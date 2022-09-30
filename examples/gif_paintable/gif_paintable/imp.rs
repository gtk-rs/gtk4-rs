use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};

use super::Frame;

#[derive(Default)]
pub struct GifPaintable {
    pub frames: RefCell<Option<Vec<Frame>>>,
    pub next_frame: RefCell<Option<gdk::Texture>>,
    pub timeout_source_id: RefCell<Option<glib::SourceId>>,
    pub current_idx: Cell<usize>,
}

#[glib::object_subclass]
impl ObjectSubclass for GifPaintable {
    const NAME: &'static str = "GifPaintable";
    type Type = super::GifPaintable;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for GifPaintable {}

impl PaintableImpl for GifPaintable {
    fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
        self.next_frame
            .borrow()
            .as_ref()
            .map(|texture| texture.height())
            .unwrap_or(-1)
    }

    fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
        self.next_frame
            .borrow()
            .as_ref()
            .map(|texture| texture.width())
            .unwrap_or(-1)
    }

    fn snapshot(&self, _paintable: &Self::Type, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        if let Some(texture) = &*self.next_frame.borrow() {
            texture.snapshot(snapshot, width, height);
        } else {
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            snapshot.append_color(
                &gdk::RGBA::BLACK,
                &graphene::Rect::new(0f32, 0f32, width as f32, height as f32),
            );
        }
    }
}
