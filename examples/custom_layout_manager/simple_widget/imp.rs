use crate::custom_layout::CustomLayout;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct SimpleWidget {
    pub backward: Cell<bool>,
    pub tick_id: RefCell<Option<gtk::TickCallbackId>>,
    pub start_time: RefCell<Option<std::time::Instant>>,
}

impl Default for SimpleWidget {
    fn default() -> Self {
        Self {
            backward: Cell::new(false),
            tick_id: RefCell::default(),
            start_time: RefCell::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleWidget {
    const NAME: &'static str = "SimpleWidget";
    type Type = super::SimpleWidget;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // We make use of the custom layout manager
        klass.set_layout_manager_type::<CustomLayout>();
    }
}

impl ObjectImpl for SimpleWidget {
    fn constructed(&self) {
        let gesture = gtk::GestureClick::new();
        // Trigger a transition on click
        let obj = self.obj();
        gesture.connect_pressed(clone!(@strong obj as this => move |_, _, _, _| {
            this.do_transition();
        }));
        self.obj().add_controller(&gesture);
    }

    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for SimpleWidget {}
