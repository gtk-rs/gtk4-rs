use gtk::glib::{self, Properties};
use gtk::glib::{ParamSpec, Value};
use gtk::gsk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use std::cell::{Cell, RefCell};

fn child_size(child: &impl IsA<gtk::Widget>) -> ((i32, i32), (i32, i32)) {
    let (horizontal_minimal, horizontal_natural, _, _) =
        child.measure(gtk::Orientation::Horizontal, -1);
    let (vertical_minimal, vertical_natural, _, _) = child.measure(gtk::Orientation::Vertical, -1);

    (
        (horizontal_minimal, horizontal_natural),
        (vertical_minimal, vertical_natural),
    )
}

#[derive(Debug, Default, Properties)]
#[properties(wrapper_type = super::SqueezerBin)]
pub struct SqueezerBin {
    #[property(get, explicit_notify)]
    pub(super) child: RefCell<Option<gtk::Widget>>,
    #[property(get, explicit_notify)]
    pub(super) keep_aspect_ratio: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for SqueezerBin {
    const NAME: &'static str = "SqueezerBin";
    type ParentType = gtk::Widget;
    type Type = super::SqueezerBin;
}

impl ObjectImpl for SqueezerBin {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        obj.set_halign(gtk::Align::Fill);
        obj.set_valign(gtk::Align::Fill);
        obj.set_hexpand(true);
        obj.set_vexpand(true);
    }

    fn dispose(&self) {
        if let Some(child) = self.obj().child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SqueezerBin {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        let widget = self.obj();
        if let Some(child) = widget.child() {
            let ((_, horizontal_size), (_, vertical_size)) = child_size(&*widget);

            let (mut horizontal_zoom, mut vertical_zoom) = (
                width as f32 / horizontal_size as f32,
                height as f32 / vertical_size as f32,
            );

            if widget.keep_aspect_ratio() {
                if horizontal_zoom < vertical_zoom {
                    vertical_zoom = horizontal_zoom;
                } else {
                    horizontal_zoom = vertical_zoom;
                }
            }

            let transform = gsk::Transform::new().scale(horizontal_zoom, vertical_zoom);

            child.allocate(
                (width as f32 / horizontal_zoom) as i32,
                (height as f32 / vertical_zoom) as i32,
                baseline,
                Some(transform),
            );
        }
    }
}
