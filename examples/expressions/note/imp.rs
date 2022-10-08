use glib::prelude::*;
use glib::subclass::prelude::*;
use gtk::glib;

use once_cell::unsync::OnceCell;

use super::Metadata;

#[derive(Debug, Default)]
pub struct Note {
    pub metadata: OnceCell<Metadata>,
}

#[glib::object_subclass]
impl ObjectSubclass for Note {
    const NAME: &'static str = "Note";
    type Type = super::Note;
}

impl ObjectImpl for Note {
    fn properties() -> &'static [glib::ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecObject::builder::<Metadata>("metadata")
                .construct_only()
                .build()]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "metadata" => {
                let metadata = value.get().unwrap();
                self.metadata.set(metadata).unwrap();
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "metadata" => self.metadata.get().unwrap().to_value(),
            _ => unimplemented!(),
        }
    }
}
