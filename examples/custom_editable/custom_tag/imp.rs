use glib::clone;
use glib::once_cell::sync::Lazy;
use glib::subclass::Signal;
use glib::{ParamSpec, Properties};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Debug, Properties)]
#[properties(wrapper_type = super::CustomTag)]
pub struct CustomTag {
    pub container: gtk::Box,
    pub button: RefCell<Option<gtk::Button>>,
    #[property(get = Self::label, set = Self::set_label, type=glib::GString)]
    label: gtk::Label,
    #[property(get, set = Self::set_has_close_button)]
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
        Self::derived_properties()
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
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
        let tag = self.obj();
        self.container.set_parent(&*tag);
        self.container.append(&self.label);

        let gesture = gtk::GestureClick::new();
        gesture.connect_released(clone!(@weak tag => move |_gesture, _n_press, _x, _y| {
            tag.emit_by_name::<()>("clicked", &[]);
        }));
        tag.add_controller(gesture);
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

impl CustomTag {
    fn label(&self) -> glib::GString {
        self.label.text()
    }
    fn set_label(&self, text: &str) {
        self.label.set_text(text)
    }

    pub fn set_has_close_button(&self, has_close_button: bool) {
        if self.has_close_button.get() == has_close_button {
            return;
        }

        if has_close_button {
            let button = gtk::Button::builder()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .has_frame(false)
                .build();
            button.connect_clicked(clone!(@weak self as tag => move |_btn| {
                tag.obj().emit_by_name::<()>("closed", &[]);
            }));
            let icon = gtk::Image::from_icon_name("window-close-symbolic");
            button.set_child(Some(&icon));

            self.container.append(&button);
            self.button.replace(Some(button));
        } else if let Some(button) = self.button.borrow_mut().take() {
            self.container.remove(&button);
        }
        self.has_close_button.set(has_close_button);
    }
}
