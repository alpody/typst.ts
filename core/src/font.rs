use std::sync::{Arc, Mutex};

use comemo::Prehashed;
use once_cell::sync::OnceCell;
use typst::{
    font::{Font, FontBook},
    util::Buffer,
};

pub trait FontResolver {
    fn font_book(&self) -> &Prehashed<FontBook>;
    fn font(&self, idx: usize) -> Option<Font>;
}

pub struct FontResolverImpl {
    book: Prehashed<FontBook>,
    fonts: Vec<FontSlot>,
}

impl FontResolverImpl {
    pub fn new(book: FontBook, fonts: Vec<FontSlot>) -> Self {
        Self {
            book: Prehashed::new(book),
            fonts,
        }
    }
}

impl FontResolver for FontResolverImpl {
    fn font_book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn font(&self, idx: usize) -> Option<Font> {
        self.fonts[idx].get()
    }
}

/// Lazy Font Reference, load as needed.
pub struct FontSlot(
    /// reference to the loaded font
    OnceCell<Option<Font>>,
    /// reference to the font loader
    Arc<Mutex<Option<Box<dyn FontLoader>>>>,
);

impl FontSlot {
    pub fn with_value(f: Option<Font>) -> Self {
        Self(OnceCell::with_value(f), Arc::new(Mutex::new(None)))
    }

    pub fn new(f: Box<dyn FontLoader>) -> Self {
        Self(OnceCell::new(), Arc::new(Mutex::new(Some(f))))
    }

    fn load(&self) -> Option<Font> {
        let mut init_fn = self.1.lock().unwrap();
        init_fn
            .take()
            .expect("the initialization fn is already poisoned")
            .load()
    }

    pub fn get(&self) -> Option<Font> {
        self.0.get_or_init(|| self.load()).clone()
    }
}

pub trait FontLoader {
    fn load(&mut self) -> Option<Font>;
}

pub struct BufferFontLoader {
    pub buffer: Option<Buffer>,
    pub index: u32,
}

impl FontLoader for BufferFontLoader {
    fn load(&mut self) -> Option<Font> {
        Font::new(self.buffer.take().unwrap(), self.index)
    }
}
