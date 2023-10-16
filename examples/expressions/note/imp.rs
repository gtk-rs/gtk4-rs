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

#[glib::derived_properties]
impl ObjectImpl for Note {}
