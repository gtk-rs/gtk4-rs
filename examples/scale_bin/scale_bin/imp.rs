use gtk::glib;
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

#[derive(Debug)]
pub struct ScaleBin {
    pub zoom: Cell<f64>,
}

#[glib::object_subclass]
impl ObjectSubclass for ScaleBin {
    const NAME: &'static str = "ScaleBin";
    type ParentType = gtk::Widget;
    type Type = super::ScaleBin;
    type Interfaces = ();

    fn new() -> Self {
        Self {
            zoom: Cell::new(5.),
        }
    }
}

impl ObjectImpl for ScaleBin {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_halign(gtk::Align::Start);
        obj.set_valign(gtk::Align::Start);

        let child = gtk::Label::new(Some("Hello World!"));
        child.set_parent(&*obj);
    }

    fn dispose(&self) {
        let child = self.obj().first_child().unwrap();
        child.unparent();
    }
}

impl WidgetImpl for ScaleBin {
    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        let zoom = self.zoom.get();
        let new_for_size = if for_size > 0 {
            (zoom * for_size as f64) as i32
        } else {
            for_size
        };

        let (minimum, natural, _, _) = self
            .obj()
            .first_child()
            .unwrap()
            .measure(orientation, new_for_size);

        (
            (minimum as f64 * zoom) as i32,
            (natural as f64 * zoom) as i32,
            -1,
            -1,
        )
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let zoom = self.zoom.get() as f32;

        let transform = gsk::Transform::new().scale(zoom, zoom);

        self.obj().first_child().unwrap().allocate(
            (width as f32 / zoom) as i32,
            (height as f32 / zoom) as i32,
            baseline,
            Some(&transform),
        );
    }
}
