mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct CustomEntryBuffer(ObjectSubclass<imp::CustomEntryBuffer>) @extends gtk::EntryBuffer;
}

impl Default for CustomEntryBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomEntryBuffer {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a CustomEntryBuffer")
    }
}
