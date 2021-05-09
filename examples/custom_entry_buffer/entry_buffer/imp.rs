use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct CustomEntryBuffer {
    text: Rc<RefCell<String>>,
    length: Cell<u16>,
    max_length: Cell<u16>,
}

impl Default for CustomEntryBuffer {
    fn default() -> Self {
        Self {
            text: Rc::new(RefCell::new(String::new())),
            length: Cell::new(0),
            max_length: Cell::new(0),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomEntryBuffer {
    const NAME: &'static str = "CustomEntryBuffer";
    type Type = super::CustomEntryBuffer;
    type ParentType = gtk::EntryBuffer;
}

impl ObjectImpl for CustomEntryBuffer {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpec::new_uint(
                    "length",
                    "length",
                    "Length",
                    0,
                    u16::MAX as u32,
                    0,
                    ParamFlags::READABLE,
                ),
                ParamSpec::new_uint(
                    "max-length",
                    "Maximum length",
                    "Maximum number of characters for this entry",
                    0,
                    u16::MAX as u32,
                    0,
                    ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY,
                ),
                ParamSpec::new_string(
                    "text",
                    "Text",
                    "The contents of the buffer",
                    Some(""),
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "length" => (self.length.get() as u32).to_value(),
            "max-length" => (self.max_length.get() as u32).to_value(),
            "text" => self.text.borrow().to_value(),
            _ => unreachable!(),
        }
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "length" => {
                self.length.set(value.get::<u32>().unwrap() as u16);
            }
            "max-length" => {
                self.max_length.set(value.get::<u32>().unwrap() as u16);
            }
            "text" => {
                self.text.replace(value.get().unwrap());
            }
            _ => unreachable!(),
        }
    }
}

impl EntryBufferImpl for CustomEntryBuffer {
    fn text(&self, _entry_buffer: &Self::Type) -> glib::GString {
        self.text.borrow().clone().into()
    }

    fn length(&self, _entry_buffer: &Self::Type) -> u16 {
        self.text.borrow().chars().count() as u16
    }

    fn insert_text(&self, entry_buffer: &Self::Type, _position: u16, chars: &str) -> u16 {
        self.text.borrow_mut().insert_str(0, chars);
        let n_chars = chars.chars().count() as u16;
        let new_length = self.length.get() + n_chars;
        self.length.set(new_length);

        entry_buffer.notify("text");
        entry_buffer.notify("length");
        n_chars
    }

    fn delete_text(&self, entry_buffer: &Self::Type, position: u16, n_chars: Option<u16>) -> u16 {
        let deleted_chars = n_chars.unwrap_or(u16::MAX);
        println!("{}", position);
        println!("{:#?}", self.text.borrow());
        let text = self.text.borrow().chars().skip(position as usize).collect();
        println!("{}", text);
        self.text.replace(text);

        let length = (self.length.get() - deleted_chars).max(0);
        self.length.set(length);

        entry_buffer.notify("text");
        entry_buffer.notify("length");

        deleted_chars
    }
}
