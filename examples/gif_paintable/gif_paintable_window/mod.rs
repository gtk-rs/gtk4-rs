mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use crate::gif_paintable::GifPaintable;

glib::wrapper! {
    pub struct GifPaintableWindow(ObjectSubclass<imp::GifPaintableWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
        gtk::Native, gtk::Root, gtk::ShortcutManager, gio::ActionMap, gio::ActionGroup;
}

impl GifPaintableWindow {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub async fn open_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gif_filter = gtk::FileFilter::new();
        gif_filter.add_mime_type("image/gif");
        gif_filter.set_name(Some("GIF Image"));

        let filters = gio::ListStore::new::<gtk::FileFilter>();
        filters.append(&gif_filter);

        let dialog = gtk::FileDialog::builder()
            .title("Open File")
            .accept_label("Open")
            .modal(true)
            .filters(&filters)
            .build();

        let file = dialog.open_future(Some(self)).await?;
        self.set_file(file)?;

        Ok(())
    }

    fn set_file(&self, file: gio::File) -> Result<(), Box<dyn std::error::Error>> {
        let paintable = GifPaintable::default();
        let (bytes, _) = file.load_contents(gio::Cancellable::NONE)?;
        paintable.load_from_bytes(&bytes)?;
        self.imp().picture.set_paintable(Some(&paintable));
        Ok(())
    }
}
