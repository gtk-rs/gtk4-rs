use gtk::{gdk, glib, graphene, gsk, prelude::*, subclass::prelude::*};

#[derive(Default)]
pub struct CustomPaintable {}

#[glib::object_subclass]
impl ObjectSubclass for CustomPaintable {
    const NAME: &'static str = "CustomPaintable";
    type Type = super::CustomPaintable;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for CustomPaintable {}

impl PaintableImpl for CustomPaintable {
    fn flags(&self) -> gdk::PaintableFlags {
        // Fixed size
        gdk::PaintableFlags::SIZE
    }

    fn intrinsic_width(&self) -> i32 {
        200
    }

    fn intrinsic_height(&self) -> i32 {
        200
    }

    fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        // Draw a linear gradient
        snapshot.append_linear_gradient(
            &graphene::Rect::new(0.0, 0.0, width as f32, height as f32),
            &graphene::Point::new(0f32, 0f32),
            &graphene::Point::new(width as f32, height as f32),
            &gsk::ColorStop::builder()
                .at(0.0, gdk::RGBA::RED)
                .at(0.15, gdk::RGBA::new(1.0, 127.0 / 255.0, 0.0, 1.0))
                .at(0.3, gdk::RGBA::new(1.0, 1.0, 0.0, 1.0))
                .at(0.45, gdk::RGBA::GREEN)
                .at(0.6, gdk::RGBA::BLUE)
                .at(0.75, gdk::RGBA::new(75.0 / 255.0, 0.0, 130.0 / 255.0, 1.0))
                .at(0.9, gdk::RGBA::new(143.0 / 255.0, 0.0, 1.0, 1.0))
                .build(),
        );
    }
}
