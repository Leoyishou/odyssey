use std::sync::LazyLock;

use anyhow::Result;
use ::util::ResultExt;
use collections::FxHashMap;
use itertools::Itertools;
use windows::Win32::{
    Foundation::{HANDLE, HGLOBAL},
    System::{
        DataExchange::{
            CloseClipboard, CountClipboardFormats, EmptyClipboard, EnumClipboardFormats,
            GetClipboardData, GetClipboardFormatNameW, IsClipboardFormatAvailable, OpenClipboard,
            RegisterClipboardFormatW, SetClipboardData,
        },
        Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock},
        Ole::{CF_DIB, CF_HDROP, CF_UNICODETEXT},
    },
    UI::Shell::{DragQueryFileW, HDROP},
};
use windows_core::PCWSTR;

use crate::{ClipboardEntry, ClipboardItem, ClipboardString, Image, ImageFormat, hash};

// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-dragqueryfilew
const DRAGDROP_GET_FILES_COUNT: u32 = 0xFFFFFFFF;

// Clipboard formats
static CLIPBOARD_HASH_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GPUI internal text hash")));
static CLIPBOARD_METADATA_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GPUI internal metadata")));
static CLIPBOARD_SVG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("image/svg+xml")));
static CLIPBOARD_GIF_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("GIF")));
static CLIPBOARD_PNG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("PNG")));
static CLIPBOARD_JPG_FORMAT: LazyLock<u32> =
    LazyLock::new(|| register_clipboard_format(windows::core::w!("JFIF")));

static IMAGE_FORMATS_MAP: LazyLock<FxHashMap<u32, ImageFormat>> = LazyLock::new(|| {
    let mut formats_map = FxHashMap::default();
    formats_map.insert(*CLIPBOARD_PNG_FORMAT, ImageFormat::Png);
    formats_map.insert(*CLIPBOARD_GIF_FORMAT, ImageFormat::Gif);
    formats_map.insert(*CLIPBOARD_JPG_FORMAT, ImageFormat::Jpeg);
    formats_map.insert(*CLIPBOARD_SVG_FORMAT, ImageFormat::Svg);
    formats_map
});

pub(crate) fn write_to_clipboard(item: ClipboardItem) {
    with_clipboard(|| write_to_clipboard_inner(item));
}

pub(crate) fn read_from_clipboard() -> Option<ClipboardItem> {
    with_clipboard(read_all_formats).flatten()
}

// Quiet.md: read the text, an image and copied files together instead of only
// the first format listed. Screenshot tools put a bitmap (CF_DIB) and often
// the saved file (CF_HDROP) on the clipboard; the image wins over the paths,
// and copied image files paste as images.
fn read_all_formats() -> Option<ClipboardItem> {
    let available = |f: u32| unsafe { IsClipboardFormatAvailable(f) }.is_ok();
    let text = available(CF_UNICODETEXT.0 as u32).then(read_string_from_clipboard).flatten();
    let image = [*CLIPBOARD_PNG_FORMAT, *CLIPBOARD_JPG_FORMAT, *CLIPBOARD_GIF_FORMAT, *CLIPBOARD_SVG_FORMAT]
        .into_iter()
        .filter(|f| available(*f))
        .find_map(read_image_from_clipboard)
        .or_else(|| available(CF_DIB.0 as u32).then(read_dib_from_clipboard).flatten());
    let files = if available(CF_HDROP.0 as u32) { read_file_names() } else { Vec::new() };
    let mut entries = Vec::new();
    let has_text = text.is_some();
    entries.extend(text);
    match image {
        Some(image) => entries.push(image),
        None if !files.is_empty() => {
            let images: Vec<_> = files.iter().map_while(|f| read_image_file(f)).collect();
            if images.len() == files.len() { entries.extend(images) }
            else if !has_text { entries.push(ClipboardEntry::String(ClipboardString::new(files.join("\n")))) }
        }
        None => {}
    }
    if entries.is_empty() {
        log_unsupported_formats();
        return None;
    }
    Some(ClipboardItem { entries })
}

/// A device-independent bitmap as PNG: a BMP file header put in front, then
/// decoded and re-encoded.
fn read_dib_from_clipboard() -> Option<ClipboardEntry> {
    let dib = with_clipboard_data(CF_DIB.0 as u32, |data_ptr, size| unsafe {
        std::slice::from_raw_parts(data_ptr as *const u8, size).to_vec()
    })?;
    let bmp = dib_to_bmp(&dib)?;
    let bytes = convert_image_to_png_format(&bmp, ImageFormat::Bmp).log_err()?;
    let id = hash(&bytes);
    Some(ClipboardEntry::Image(Image { format: ImageFormat::Png, bytes, id }))
}

fn dib_to_bmp(dib: &[u8]) -> Option<Vec<u8>> {
    let u32_at = |i: usize| Some(u32::from_le_bytes(dib.get(i..i + 4)?.try_into().ok()?));
    let header = u32_at(0)?;
    if header < 40 || dib.len() < header as usize { return None; }
    let bits = u16::from_le_bytes(dib.get(14..16)?.try_into().ok()?);
    let compression = u32_at(16)?;
    let used = u32_at(32)?;
    // BITMAPINFOHEADER keeps the BI_BITFIELDS masks after the header.
    let masks = match (header, compression) { (40, 3) => 12, (40, 6) => 16, _ => 0 };
    let colors = if used > 0 { used } else if bits <= 8 { 1 << bits } else { 0 };
    let offset = 14 + header + masks + colors * 4;
    let mut bmp = Vec::with_capacity(14 + dib.len());
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(14 + dib.len() as u32).to_le_bytes());
    bmp.extend_from_slice(&[0; 4]);
    bmp.extend_from_slice(&offset.to_le_bytes());
    bmp.extend_from_slice(dib);
    Some(bmp)
}

fn read_file_names() -> Vec<String> {
    with_clipboard_data(CF_HDROP.0 as u32, |data_ptr, _size| {
        let mut names = Vec::new();
        with_file_names(HDROP(data_ptr), |name| names.push(name));
        names
    })
    .unwrap_or_default()
}

/// A copied image file, read as the image it holds.
fn read_image_file(path: &str) -> Option<ClipboardEntry> {
    let ext = std::path::Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    let format = match ext.as_str() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "gif" => ImageFormat::Gif,
        "webp" => ImageFormat::Webp,
        "bmp" => ImageFormat::Bmp,
        "svg" => ImageFormat::Svg,
        _ => return None,
    };
    let bytes = std::fs::read(path).log_err()?;
    let id = hash(&bytes);
    Some(ClipboardEntry::Image(Image { format, bytes, id }))
}

pub(crate) fn with_file_names<F>(hdrop: HDROP, mut f: F)
where
    F: FnMut(String),
{
    let file_count = unsafe { DragQueryFileW(hdrop, DRAGDROP_GET_FILES_COUNT, None) };
    for file_index in 0..file_count {
        let filename_length = unsafe { DragQueryFileW(hdrop, file_index, None) } as usize;
        let mut buffer = vec![0u16; filename_length + 1];
        let ret = unsafe { DragQueryFileW(hdrop, file_index, Some(buffer.as_mut_slice())) };
        if ret == 0 {
            log::error!("unable to read file name of dragged file");
            continue;
        }
        match String::from_utf16(&buffer[0..filename_length]) {
            Ok(file_name) => f(file_name),
            Err(e) => {
                log::error!("dragged file name is not UTF-16: {}", e)
            }
        }
    }
}

fn with_clipboard<F, T>(f: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    match unsafe { OpenClipboard(None) } {
        Ok(()) => {
            let result = f();
            if let Err(e) = unsafe { CloseClipboard() } {
                log::error!("Failed to close clipboard: {e}",);
            }
            Some(result)
        }
        Err(e) => {
            log::error!("Failed to open clipboard: {e}",);
            None
        }
    }
}

fn register_clipboard_format(format: PCWSTR) -> u32 {
    let ret = unsafe { RegisterClipboardFormatW(format) };
    if ret == 0 {
        panic!(
            "Error when registering clipboard format: {}",
            std::io::Error::last_os_error()
        );
    }
    ret
}

// Currently, we only write the first item.
fn write_to_clipboard_inner(item: ClipboardItem) -> Result<()> {
    unsafe {
        EmptyClipboard()?;
    }
    match item.entries().first() {
        Some(entry) => match entry {
            ClipboardEntry::String(string) => {
                write_string_to_clipboard(string)?;
            }
            ClipboardEntry::Image(image) => {
                write_image_to_clipboard(image)?;
            }
        },
        None => {
            // Writing an empty list of entries just clears the clipboard.
        }
    }
    Ok(())
}

fn write_string_to_clipboard(item: &ClipboardString) -> Result<()> {
    let encode_wide = item.text.encode_utf16().chain(Some(0)).collect_vec();
    set_data_to_clipboard(&encode_wide, CF_UNICODETEXT.0 as u32)?;

    if let Some(metadata) = item.metadata.as_ref() {
        let hash_result = {
            let hash = ClipboardString::text_hash(&item.text);
            hash.to_ne_bytes()
        };
        let encode_wide =
            unsafe { std::slice::from_raw_parts(hash_result.as_ptr().cast::<u16>(), 4) };
        set_data_to_clipboard(encode_wide, *CLIPBOARD_HASH_FORMAT)?;

        let metadata_wide = metadata.encode_utf16().chain(Some(0)).collect_vec();
        set_data_to_clipboard(&metadata_wide, *CLIPBOARD_METADATA_FORMAT)?;
    }
    Ok(())
}

fn set_data_to_clipboard<T>(data: &[T], format: u32) -> Result<()> {
    unsafe {
        let global = GlobalAlloc(GMEM_MOVEABLE, std::mem::size_of_val(data))?;
        let handle = GlobalLock(global);
        std::ptr::copy_nonoverlapping(data.as_ptr(), handle as _, data.len());
        let _ = GlobalUnlock(global);
        SetClipboardData(format, Some(HANDLE(global.0)))?;
    }
    Ok(())
}

// Here writing PNG to the clipboard to better support other apps. For more info, please ref to
// the PR.
fn write_image_to_clipboard(item: &Image) -> Result<()> {
    match item.format {
        ImageFormat::Svg => set_data_to_clipboard(item.bytes(), *CLIPBOARD_SVG_FORMAT)?,
        ImageFormat::Gif => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_GIF_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Gif)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        ImageFormat::Png => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_PNG_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Png)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        ImageFormat::Jpeg => {
            set_data_to_clipboard(item.bytes(), *CLIPBOARD_JPG_FORMAT)?;
            let png_bytes = convert_image_to_png_format(item.bytes(), ImageFormat::Jpeg)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
        other => {
            log::warn!(
                "Clipboard unsupported image format: {:?}, convert to PNG instead.",
                item.format
            );
            let png_bytes = convert_image_to_png_format(item.bytes(), other)?;
            set_data_to_clipboard(&png_bytes, *CLIPBOARD_PNG_FORMAT)?;
        }
    }
    Ok(())
}

fn convert_image_to_png_format(bytes: &[u8], image_format: ImageFormat) -> Result<Vec<u8>> {
    let image = image::load_from_memory_with_format(bytes, image_format.into())?;
    let mut output_buf = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut output_buf),
        image::ImageFormat::Png,
    )?;
    Ok(output_buf)
}

fn log_unsupported_formats() {
    let count = unsafe { CountClipboardFormats() };
    let mut clipboard_format = 0;
    for _ in 0..count {
        clipboard_format = unsafe { EnumClipboardFormats(clipboard_format) };
        let mut buffer = [0u16; 64];
        unsafe { GetClipboardFormatNameW(clipboard_format, &mut buffer) };
        let format_name = String::from_utf16_lossy(&buffer);
        log::warn!(
            "Try to paste with unsupported clipboard format: {}, {}.",
            clipboard_format,
            format_name
        );
    }
}

fn read_string_from_clipboard() -> Option<ClipboardEntry> {
    let text = with_clipboard_data(CF_UNICODETEXT.0 as u32, |data_ptr, _| {
        let pcwstr = PCWSTR(data_ptr as *const u16);
        String::from_utf16_lossy(unsafe { pcwstr.as_wide() })
    })?;
    let Some(hash) = read_hash_from_clipboard() else {
        return Some(ClipboardEntry::String(ClipboardString::new(text)));
    };
    let Some(metadata) = read_metadata_from_clipboard() else {
        return Some(ClipboardEntry::String(ClipboardString::new(text)));
    };
    if hash == ClipboardString::text_hash(&text) {
        Some(ClipboardEntry::String(ClipboardString {
            text,
            metadata: Some(metadata),
        }))
    } else {
        Some(ClipboardEntry::String(ClipboardString::new(text)))
    }
}

fn read_hash_from_clipboard() -> Option<u64> {
    if unsafe { IsClipboardFormatAvailable(*CLIPBOARD_HASH_FORMAT).is_err() } {
        return None;
    }
    with_clipboard_data(*CLIPBOARD_HASH_FORMAT, |data_ptr, size| {
        if size < 8 {
            return None;
        }
        let hash_bytes: [u8; 8] = unsafe {
            std::slice::from_raw_parts(data_ptr.cast::<u8>(), 8)
                .try_into()
                .ok()
        }?;
        Some(u64::from_ne_bytes(hash_bytes))
    })?
}

fn read_metadata_from_clipboard() -> Option<String> {
    unsafe { IsClipboardFormatAvailable(*CLIPBOARD_METADATA_FORMAT).ok()? };
    with_clipboard_data(*CLIPBOARD_METADATA_FORMAT, |data_ptr, _size| {
        let pcwstr = PCWSTR(data_ptr as *const u16);
        String::from_utf16_lossy(unsafe { pcwstr.as_wide() })
    })
}

fn read_image_from_clipboard(format: u32) -> Option<ClipboardEntry> {
    let image_format = format_number_to_image_format(format)?;
    read_image_for_type(format, *image_format)
}

#[inline]
fn format_number_to_image_format(format_number: u32) -> Option<&'static ImageFormat> {
    IMAGE_FORMATS_MAP.get(&format_number)
}

fn read_image_for_type(format_number: u32, format: ImageFormat) -> Option<ClipboardEntry> {
    let (bytes, id) = with_clipboard_data(format_number, |data_ptr, size| {
        let bytes = unsafe { std::slice::from_raw_parts(data_ptr as *mut u8 as _, size).to_vec() };
        let id = hash(&bytes);
        (bytes, id)
    })?;
    Some(ClipboardEntry::Image(Image { format, bytes, id }))
}

fn with_clipboard_data<F, R>(format: u32, f: F) -> Option<R>
where
    F: FnOnce(*mut std::ffi::c_void, usize) -> R,
{
    let global = HGLOBAL(unsafe { GetClipboardData(format).ok() }?.0);
    let size = unsafe { GlobalSize(global) };
    let data_ptr = unsafe { GlobalLock(global) };
    let result = f(data_ptr, size);
    unsafe { GlobalUnlock(global).ok() };
    Some(result)
}

impl From<ImageFormat> for image::ImageFormat {
    fn from(value: ImageFormat) -> Self {
        match value {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Webp => image::ImageFormat::WebP,
            ImageFormat::Gif => image::ImageFormat::Gif,
            // TODO: ImageFormat::Svg
            ImageFormat::Bmp => image::ImageFormat::Bmp,
            ImageFormat::Tiff => image::ImageFormat::Tiff,
            _ => unreachable!(),
        }
    }
}
