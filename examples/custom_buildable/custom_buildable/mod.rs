mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomBuildable(ObjectSubclass<imp::CustomBuildable>) @extends gtk::Widget, @implements gtk::Buildable;
}

impl CustomBuildable {
    pub fn add_suffix<T: glib::IsA<gtk::Widget>>(&self, widget: &T) {
        let imp = self.imp();
        imp.suffixes.append(widget);
        imp.suffixes.show();
    }

    pub fn add_prefix<T: glib::IsA<gtk::Widget>>(&self, widget: &T) {
        let imp = self.imp();
        imp.prefixes.append(widget);
        imp.prefixes.show();
    }
}
