//! Other platforms use the sidebar surface painted by GPUI.
pub fn supported() -> bool { false }

#[derive(Default)]
pub struct NativeSidebar;
impl NativeSidebar {
    pub fn sync(&self, _: f32, _: &gpui::Window) {}
}
