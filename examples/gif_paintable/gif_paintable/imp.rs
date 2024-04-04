use std::cell::{Cell, RefCell};

use gtk::{gdk, glib, graphene, prelude::*, subclass::prelude::*};

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
    fn intrinsic_height(&self) -> i32 {
        self.next_frame
            .borrow()
            .as_ref()
            .map(|texture| texture.height())
            .unwrap_or(-1)
    }

    fn intrinsic_width(&self) -> i32 {
        self.next_frame
            .borrow()
            .as_ref()
            .map(|texture| texture.width())
            .unwrap_or(-1)
    }

    fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        if let Some(texture) = &*self.next_frame.borrow() {
            texture.snapshot(snapshot, width, height);
        } else {
            snapshot.append_color(
                &gdk::RGBA::BLACK,
                &graphene::Rect::new(0f32, 0f32, width as f32, height as f32),
            );
        }
    }
}
