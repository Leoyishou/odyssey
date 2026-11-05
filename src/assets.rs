//! Assets compiled into the binary: Lucide icons (ISC licensed).

use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if path.starts_with(crate::editor::math::SVG_PREFIX) {
            return Ok(crate::editor::math::svg_asset(path).map(Cow::Owned));
        }
        Ok(FILES.iter().find(|(p, _)| *p == path).map(|(_, b)| Cow::Borrowed(*b)))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(FILES.iter().filter(|(p, _)| p.starts_with(path)).map(|(p, _)| SharedString::from(*p)).collect())
    }
}
