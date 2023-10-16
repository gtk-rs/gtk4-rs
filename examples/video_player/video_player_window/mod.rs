mod imp;

use gtk::{gio, glib, subclass::prelude::*};

glib::wrapper! {
    pub struct VideoPlayerWindow(ObjectSubclass<imp::VideoPlayerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl VideoPlayerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn set_video(&self, video: gio::File) {
        self.imp().video.set_file(Some(&video));
    }
}
