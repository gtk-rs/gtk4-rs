mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct SqueezerBin(ObjectSubclass<imp::SqueezerBin>) @extends gtk::Widget;
}

impl SqueezerBin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn keep_aspect_ratio(&self) -> bool {
        self.imp().keep_aspect_ratio(self)
    }

    pub fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        self.imp().set_keep_aspect_ratio(self, keep_aspect_ratio)
    }

    pub fn child(&self) -> Option<gtk::Widget> {
        self.imp().child(self)
    }

    pub fn set_child(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        self.imp().set_child(self, widget);
    }
}
