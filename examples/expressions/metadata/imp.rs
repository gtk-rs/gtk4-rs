use glib::prelude::*;
use glib::subclass::prelude::*;
use gtk::glib;

use std::cell::RefCell;

#[derive(Debug)]
pub struct Metadata {
    pub title: RefCell<String>,
    pub last_modified: RefCell<glib::DateTime>,
}

#[glib::object_subclass]
impl ObjectSubclass for Metadata {
    const NAME: &'static str = "Metadata";
    type Type = super::Metadata;

    fn new() -> Self {
        let now = glib::DateTime::new_now_local().unwrap();

        Self {
            title: RefCell::new(String::new()),
            last_modified: RefCell::new(now),
        }
    }
}

impl ObjectImpl for Metadata {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpec::new_string(
                    "title",
                    "Title",
                    "Title of the note",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpec::new_boxed(
                    "last-modified",
                    "Last Modified",
                    "Last modified datetime of the note",
                    glib::DateTime::static_type(),
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) {
        match pspec.name() {
            "title" => {
                let title = value.get().unwrap();
                self.title.replace(title);
            }
            "last-modified" => {
                let last_modified = value.get().unwrap();
                self.last_modified.replace(last_modified);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "title" => self.title.borrow().to_value(),
            "last-modified" => self.last_modified.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
