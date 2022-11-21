use glib::{ParamSpec, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Debug)]
pub struct CustomLayoutChild {
    pub color: RefCell<gdk::RGBA>,
}

impl Default for CustomLayoutChild {
    fn default() -> Self {
        Self {
            color: RefCell::new(gdk::RGBA::BLACK),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomLayoutChild {
    const NAME: &'static str = "CustomLayoutChild";
    type Type = super::CustomLayoutChild;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for CustomLayoutChild {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![glib::ParamSpecBoxed::builder::<gdk::RGBA>("color")
                .construct_only()
                .build()]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "color" => self.color.borrow().to_value(),
            _ => unreachable!(),
        }
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "color" => self.color.replace(value.get().unwrap()),
            _ => unreachable!(),
        };
    }

    fn constructed(&self) {
        self.parent_constructed();
        let widget = self.obj();
        widget.set_margin_top(4);
        widget.set_margin_bottom(4);
        widget.set_margin_start(4);
        widget.set_margin_end(4);
    }
}

impl WidgetImpl for CustomLayoutChild {
    fn measure(&self, _orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
        // Return (minimum size, natural size, baseline position for the minimum size, baseline position for natural size)
        (32, 32, -1, -1)
    }

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let widget = self.obj();
        snapshot.append_color(
            &self.color.borrow(),
            &graphene::Rect::new(0.0, 0.0, widget.width() as f32, widget.height() as f32),
        )
    }
}
