use glib::clone;
use glib::subclass::Signal;
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecString};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

#[derive(Debug)]
pub struct CustomTag {
    pub container: gtk::Box,
    pub button: RefCell<Option<gtk::Button>>,
    label: gtk::Label,
    pub has_close_button: Cell<bool>,
}

impl Default for CustomTag {
    fn default() -> Self {
        Self {
            container: gtk::Box::new(gtk::Orientation::Horizontal, 0),
            button: RefCell::default(),
            label: gtk::Label::new(None),
            has_close_button: Cell::new(false),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for CustomTag {
    const NAME: &'static str = "CustomTag";
    type Type = super::CustomTag;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_css_name("tag");
    }
}

impl ObjectImpl for CustomTag {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("label").build(),
                ParamSpecBoolean::builder("has-close-button").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "label" => self.label.text().to_value(),
            "has-close-button" => self.has_close_button.get().to_value(),
            _ => unimplemented!(),
        }
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "label" => self.label.set_text(value.get().unwrap()),
            "has-close-button" => {
                self.instance().set_has_close_button(value.get().unwrap());
            }
            _ => unimplemented!(),
        }
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("closed").build(),
                Signal::builder("clicked").build(),
            ]
        });
        SIGNALS.as_ref()
    }

    fn constructed(&self) {
        self.parent_constructed();
        let tag = self.instance();
        self.container.set_parent(&*tag);
        self.container.append(&self.label);

        let gesture = gtk::GestureClick::new();
        gesture.connect_released(clone!(@weak tag => move |_gesture, _n_press, _x, _y| {
            tag.emit_by_name::<()>("clicked", &[]);
        }));
        tag.add_controller(&gesture);
    }

    fn dispose(&self) {
        self.container.unparent();
    }
}
impl WidgetImpl for CustomTag {
    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        self.container.measure(orientation, for_size)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.container
            .size_allocate(&gtk::Allocation::new(0, 0, width, height), baseline)
    }
}
