mod imp;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct CustomBuildable(ObjectSubclass<imp::CustomBuildable>) @extends gtk::Widget, @implements gtk::Buildable;
}

impl CustomBuildable {
    pub fn add_suffix<T: glib::IsA<gtk::Widget>>(&self, widget: &T) {
        let self_ = imp::CustomBuildable::from_instance(self);
        self_.suffixes.append(widget);
        self_.suffixes.show();
    }

    pub fn add_prefix<T: glib::IsA<gtk::Widget>>(&self, widget: &T) {
        let self_ = imp::CustomBuildable::from_instance(self);
        self_.prefixes.append(widget);
        self_.prefixes.show();
    }
}
