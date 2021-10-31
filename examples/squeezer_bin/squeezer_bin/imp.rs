use gtk::glib;
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Debug)]
pub struct SqueezerBin {
    pub keep_aspect_ratio: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for SqueezerBin {
    const NAME: &'static str = "SqueezerBin";
    type ParentType = gtk::Widget;
    type Type = super::SqueezerBin;

    fn new() -> Self {
        Self {
            keep_aspect_ratio: Cell::new(false),
        }
    }
}

impl ObjectImpl for SqueezerBin {
    fn constructed(&self, obj: &Self::Type) {
        obj.set_halign(gtk::Align::Fill);
        obj.set_valign(gtk::Align::Fill);
        obj.set_hexpand(true);
        obj.set_vexpand(true);

        let child = gtk::Label::new(Some("Hello World!"));
        child.set_parent(obj);

        self.parent_constructed(obj);
    }

    fn dispose(&self, obj: &Self::Type) {
        let child = obj.first_child().unwrap();
        child.unparent();
    }
}

impl WidgetImpl for SqueezerBin {
    fn size_allocate(&self, widget: &Self::Type, width: i32, height: i32, baseline: i32) {
        let ((_, horizontal), (_, vertical)) = widget.get_child_size();

        let (mut horizontal_zoom, mut vertical_zoom) = (
            width as f32 / horizontal as f32,
            height as f32 / vertical as f32,
        );

        if self.keep_aspect_ratio.get() {
            if horizontal_zoom < vertical_zoom {
                vertical_zoom = horizontal_zoom;
            } else {
                horizontal_zoom = vertical_zoom;
            }
        }

        let transform = gsk::Transform::new()
            .scale(horizontal_zoom, vertical_zoom)
            .unwrap();

        widget.first_child().unwrap().allocate(
            (width as f32 / horizontal_zoom) as i32,
            (height as f32 / vertical_zoom) as i32,
            baseline,
            Some(&transform),
        );
    }
}
