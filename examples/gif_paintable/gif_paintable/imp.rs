use std::cell::{Cell, RefCell};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};

use super::Frame;

pub struct GifPaintable {
    pub frames: RefCell<Option<Vec<Frame>>>,
    pub next_frame: RefCell<Option<gdk::Paintable>>,
    pub width: Cell<Option<i32>>,
    pub height: Cell<Option<i32>>,
    pub current_idx: Cell<usize>,
}

#[glib::object_subclass]
impl ObjectSubclass for GifPaintable {
    const NAME: &'static str = "GifPaintable";
    type Type = super::GifPaintable;
    type Interfaces = (gdk::Paintable,);

    fn new() -> Self {
        Self {
            next_frame: RefCell::new(None),
            frames: RefCell::new(None),
            width: Cell::new(Some(0)),
            height: Cell::new(Some(0)),
            current_idx: Cell::new(0),
        }
    }
}

impl ObjectImpl for GifPaintable {}

impl PaintableImpl for GifPaintable {
    fn intrinsic_height(&self, _paintable: &Self::Type) -> i32 {
        self.height.get().unwrap_or(-1)
    }

    fn intrinsic_width(&self, _paintable: &Self::Type) -> i32 {
        self.width.get().unwrap_or(-1)
    }

    fn snapshot(&self, _paintable: &Self::Type, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        if let (Some(texture), Some(width), Some(height)) = (
            self.next_frame.borrow_mut().take(),
            self.width.take(),
            self.height.take(),
        ) {
            let w = width as f64;
            let h = height as f64;
            texture.snapshot(snapshot, w, h);
        } else {
            let snapshot = snapshot.downcast_ref::<gtk::Snapshot>().unwrap();
            snapshot.append_color(
                &gdk::RGBA::BLACK,
                &graphene::Rect::new(0f32, 0f32, width as f32, height as f32),
            );
        }
    }
}
