use glib::prelude::*;
use glib::subclass::prelude::*;
use gtk::glib::{self, Properties};

use glib::once_cell::unsync::OnceCell;

use super::Metadata;

#[derive(Debug, Properties, Default)]
#[properties(wrapper_type = super::Note)]
pub struct Note {
    #[property(get, set, construct_only)]
    pub metadata: OnceCell<Metadata>,
}

#[glib::object_subclass]
impl ObjectSubclass for Note {
    const NAME: &'static str = "Note";
    type Type = super::Note;
}

impl ObjectImpl for Note {
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
