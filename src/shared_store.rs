use crate::store::*;
use crate::model::Document;
use anyhow::Result;
use std::{rc::Rc, cell::RefCell};

#[derive(Clone)]
pub struct SharedStore(Rc<RefCell<Store>>);
impl gpui::Global for SharedStore {}
impl SharedStore {
    pub fn open() -> Result<Self> { Ok(Self(Rc::new(RefCell::new(Store::open()?)))) }
    pub fn take_warnings(&self) -> Vec<String> { std::mem::take(&mut self.0.borrow_mut().warnings) }
    pub fn list(&mut self) -> Vec<Entry> { self.0.borrow_mut().list() }
    pub fn load(&self, id: &str) -> Result<Document> { self.0.borrow_mut().load(id) }
    pub fn load_for_edit(&self, id: &str, shared: bool) -> Result<Document> { self.0.borrow().load_for_edit(id, shared) }
    pub fn save(&mut self, id: &str, doc: &Document) -> Result<Meta> { self.0.borrow_mut().save(id, doc) }
    pub fn create_pending(&mut self, id: &str, title: &str) -> Result<()> { self.0.borrow_mut().create_pending(id, title) }
    pub fn create(&mut self) -> Result<(String, Document)> { self.0.borrow_mut().create() }
    pub fn create_from(&mut self, doc: &Document) -> Result<String> { self.0.borrow_mut().create_from(doc) }
    pub fn size_of(&self, id: &str, doc: &Document) -> u64 { self.0.borrow_mut().size_of(id, doc) }
    pub fn delete(&mut self, id: &str, root: Option<&str>) -> Result<()> { self.0.borrow_mut().delete(id, root) }
    pub fn list_trash(&mut self) -> Vec<TrashEntry> { self.0.borrow_mut().list_trash() }
    pub fn restore(&mut self, id: &str) -> Result<bool> { self.0.borrow_mut().restore(id) }
    pub fn purge(&mut self, id: &str) -> Result<()> { self.0.borrow_mut().purge(id) }
    pub fn import(&mut self, path: &std::path::Path) -> Result<String> { self.0.borrow_mut().import(path) }
    pub fn set_width(&mut self, id: &str, width: PageWidth) -> Result<()> { self.0.borrow_mut().set_width(id, width) }
    pub fn update_meta(&mut self, id: &str, f: impl FnOnce(&mut Meta)) -> Result<()> { self.0.borrow_mut().update_meta(id, f) }
    pub fn set_catalogue(&mut self, id: &str, pinned: bool) -> Result<()> { self.0.borrow_mut().set_catalogue(id, pinned) }
    pub fn settings(&self) -> Settings { self.0.borrow_mut().settings() }
    pub fn save_settings(&self, s: &Settings) -> Result<()> { self.0.borrow_mut().save_settings(s) }
    pub fn rename(&mut self, id: &str, title: &str) -> Result<Meta> { self.0.borrow_mut().rename(id, title) }
    pub fn touch(&mut self, id: &str) -> Option<Meta> { self.0.borrow_mut().touch(id) }
    pub fn set_parent(&mut self, id: &str, parent: Option<&str>) -> Result<Meta> { self.0.borrow_mut().set_parent(id, parent) }
    pub fn set_starred(&mut self, id: &str, on: bool) -> Result<u64> { self.0.borrow_mut().set_starred(id, on) }
    pub fn set_pinned(&mut self, id: &str, pinned: bool) -> Result<()> { self.0.borrow_mut().set_pinned(id, pinned) }
}
