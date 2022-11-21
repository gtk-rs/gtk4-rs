mod imp;

use gtk::glib;

use super::Metadata;

glib::wrapper! {
    pub struct Note(ObjectSubclass<imp::Note>);
}

impl Default for Note {
    fn default() -> Self {
        Self::new(&Metadata::default())
    }
}

impl Note {
    pub fn new(metadata: &Metadata) -> Self {
        glib::Object::new(&[("metadata", metadata)])
    }
}
