mod imp;

use gtk::{glib, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct SqueezerBin(ObjectSubclass<imp::SqueezerBin>)
        @extends gtk::Widget;
}

impl Default for SqueezerBin {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl SqueezerBin {
    pub fn set_child(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        let imp = self.imp();
        let widget = widget.map(|w| w.as_ref());
        if widget == imp.child.borrow().as_ref() {
            return;
        }

        if let Some(child) = imp.child.borrow_mut().take() {
            child.unparent();
        }

        if let Some(w) = widget {
            imp.child.replace(Some(w.clone()));
            w.set_parent(self);
        }

        self.queue_resize();
        self.notify("child")
    }

    pub fn set_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        let imp = self.imp();
        if imp.keep_aspect_ratio.get() == keep_aspect_ratio {
            return;
        }

        imp.keep_aspect_ratio.set(keep_aspect_ratio);

        self.queue_resize();
        self.notify("keep-aspect-ratio")
    }
}
