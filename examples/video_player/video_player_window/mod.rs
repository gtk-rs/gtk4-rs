mod imp;

use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct VideoPlayerWindow(ObjectSubclass<imp::VideoPlayerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl VideoPlayerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create VideoPlayerWindow")
    }

    fn set_video(&self, video: gio::File) {
        let self_ = imp::VideoPlayerWindow::from_instance(self);
        self_.video.set_file(Some(&video));
    }
}
