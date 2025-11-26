use gtk::{gdk, glib, graphene, gsk, prelude::*, subclass::prelude::*};

pub struct CustomPaintable {
    pub(super) height: i32,
    pub(super) width: i32,
    pub(super) paths: Vec<gsk::Path>,
    pub(super) colors: Vec<gdk::RGBA>,
    pub(super) stroke_path: gsk::Path,
    pub(super) stroke1: gsk::Stroke,
    pub(super) stroke2: gsk::Stroke,
    pub(super) stroke_color: gdk::RGBA,
}

impl Default for CustomPaintable {
    fn default() -> Self {
        // Paths and colors extracted from gtk-logo.svg
        let paths = vec![
            gsk::Path::parse("m3.12,66.17 -2.06,-51.46 32.93,24.7 v55.58 l-30.87,-28.82 z")
                .unwrap(),
            gsk::Path::parse("m34,95 49.4,-20.58 4.12,-51.46 -53.52,16.47 v55.58 z").unwrap(),
            gsk::Path::parse("m1.06,14.71 32.93,24.7 53.52,-16.47 -36.75,-21.88 -49.7,13.65 z")
                .unwrap(),
        ];
        let colors = vec![
            gdk::RGBA::parse("#e40000").unwrap(),
            gdk::RGBA::parse("#7fe719").unwrap(),
            gdk::RGBA::parse("#729fcf").unwrap(),
        ];
        let stroke_path = gsk::Path::parse("m50.6,51.3 -47.3,14 z l33,23 z v-50").unwrap();
        let stroke1 = gsk::Stroke::new(2.12);
        let stroke2 = gsk::Stroke::new(1.25);
        let stroke_color = gdk::RGBA::WHITE;

        let mut bounds = paths[0].stroke_bounds(&stroke1).unwrap();
        let bounds2 = paths[1].stroke_bounds(&stroke1).unwrap();
        bounds = bounds.union(&bounds2);
        let bounds3 = paths[2].stroke_bounds(&stroke1).unwrap();
        bounds = bounds.union(&bounds3);
        let bounds_path = stroke_path.stroke_bounds(&stroke2).unwrap();
        bounds = bounds.union(&bounds_path);
        let width = bounds.x() + bounds.width();
        let height = bounds.y() + bounds.height();
        Self {
            height: height as i32,
            width: width as i32,
            paths,
            colors,
            stroke_path,
            stroke1,
            stroke2,
            stroke_color,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomPaintable {
    const NAME: &'static str = "CustomPaintable";
    type Type = super::CustomPaintable;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for CustomPaintable {}

impl PaintableImpl for CustomPaintable {
    fn flags(&self) -> gdk::PaintableFlags {
        // Fixed size and content
        gdk::PaintableFlags::STATIC_SIZE | gdk::PaintableFlags::STATIC_CONTENTS
    }

    fn intrinsic_width(&self) -> i32 {
        self.width
    }

    fn intrinsic_height(&self) -> i32 {
        self.height
    }

    fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        let bounds = graphene::Rect::new(0., 0., width as f32, height as f32);
        for (i, path) in self.paths.iter().enumerate() {
            snapshot.push_fill(path, gsk::FillRule::Winding);
            snapshot.append_color(&self.colors[i], &bounds);
            snapshot.pop();
        }

        for _ in 0..3 {
            snapshot.push_stroke(&self.stroke_path, &self.stroke1);
            snapshot.append_color(&self.stroke_color, &bounds);
            snapshot.pop();
        }

        snapshot.push_stroke(&self.stroke_path, &self.stroke2);
        snapshot.append_color(&self.stroke_color, &bounds);
        snapshot.pop();
    }
}
