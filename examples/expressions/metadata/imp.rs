use glib::prelude::*;
use glib::subclass::prelude::*;
use gtk::glib::{self, Properties};

use std::cell::RefCell;

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::Metadata)]
pub struct Metadata {
    #[property(get, set)]
    pub title: RefCell<String>,
    #[property(get, set)]
    pub last_modified: RefCell<glib::DateTime>,
}

#[glib::object_subclass]
impl ObjectSubclass for Metadata {
    const NAME: &'static str = "Metadata";
    type Type = super::Metadata;

    fn new() -> Self {
        let now = glib::DateTime::now_local().unwrap();

        Self {
            title: RefCell::new(String::new()),
            last_modified: RefCell::new(now),
        }
    }
}

impl ObjectImpl for Metadata {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }
}
