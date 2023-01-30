use glib::{ParamSpec, Properties, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib, graphene};
use std::cell::RefCell;

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::CustomLayoutChild)]
pub struct CustomLayoutChild {
    #[property(get, set, construct_only)]
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
