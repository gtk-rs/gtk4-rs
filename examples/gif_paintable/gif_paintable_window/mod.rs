mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::gif_paintable::GifPaintable;

glib::wrapper! {
    pub struct GifPaintableWindow(ObjectSubclass<imp::GifPaintableWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl GifPaintableWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    fn set_file(&self, file: gio::File) -> Result<(), Box<dyn std::error::Error>> {
        let paintable = GifPaintable::default();
        let (bytes, _) = file.load_contents(gio::Cancellable::NONE)?;
        paintable.load_from_bytes(&bytes)?;
        self.imp().picture.set_paintable(Some(&paintable));
        Ok(())
    }
}
