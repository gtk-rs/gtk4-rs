use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub mod imp {
    use super::*;
    use glib::subclass::Signal;
    use glib::ParamSpec;
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
                    ParamSpec::string(
                        "label",
                        "Label",
                        "Label",
                        Some(""),
                        glib::ParamFlags::READWRITE,
                    ),
                    ParamSpec::boolean(
                        "has-close-button",
                        "Has close button",
                        "Whether this tag has a close button",
                        false,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn get_property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.get_name() {
                "label" => self.label.get_text().to_value(),
                "has-close-button" => self.has_close_button.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(
            &self,
            tag: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.get_name() {
                "label" => self.label.set_text(value.get().unwrap().unwrap()),
                "has-close-button" => {
                    tag.set_has_close_button(value.get_some().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("closed", &[], <()>::static_type().into()).build(),
                    Signal::builder("clicked", &[], <()>::static_type().into()).build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self, tag: &Self::Type) {
            self.container.set_parent(tag);
            self.container.append(&self.label);

            let gesture = gtk::GestureClick::new();
            gesture.connect_released(clone!(@weak tag => move |_gesture, _n_press, _x, _y| {
                tag.emit_by_name("clicked", &[]).unwrap();
            }));
            tag.add_controller(&gesture);
        }

        fn dispose(&self, _tag: &Self::Type) {
            self.container.unparent();
        }
    }
    impl WidgetImpl for CustomTag {
        fn measure(
            &self,
            _widget: &Self::Type,
            orientation: gtk::Orientation,
            for_size: i32,
            min: &mut i32,
            nat: &mut i32,
            min_base: &mut i32,
            nat_base: &mut i32,
        ) {
            let (c_min, c_nat, c_min_base, c_nat_base) =
                self.container.measure(orientation, for_size);
            *min = c_min;
            *nat = c_nat;
            *min_base = c_min_base;
            *nat_base = c_nat_base;
        }

        fn size_allocate(&self, _widget: &Self::Type, width: i32, height: i32, baseline: i32) {
            self.container.size_allocate(
                &gtk::Allocation {
                    width,
                    height,
                    x: 0,
                    y: 0,
                },
                baseline,
            )
        }
    }
}

glib::wrapper! {
    pub struct CustomTag(ObjectSubclass<imp::CustomTag>) @extends gtk::Widget;
}

impl CustomTag {
    pub fn new(label: &str) -> Self {
        glib::Object::new(&[("label", &label), ("has-close-button", &true)])
            .expect("Failed to create a CustomTag")
    }

    pub fn set_has_close_button(&self, has_close_button: bool) {
        let self_ = imp::CustomTag::from_instance(self);
        if self_.has_close_button.get() == has_close_button {
            return;
        }

        if has_close_button {
            let button = gtk::ButtonBuilder::new()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .has_frame(false)
                .build();
            button.connect_clicked(clone!(@weak self as tag => move |_btn| {
                tag.emit_by_name("closed", &[]).unwrap();
            }));
            let icon = gtk::Image::from_icon_name(Some("window-close-symbolic"));
            button.set_child(Some(&icon));

            self_.container.append(&button);
            self_.button.replace(Some(button));
        } else if let Some(button) = self_.button.borrow_mut().take() {
            self_.container.remove(&button);
        }
        self_.has_close_button.set(has_close_button);
    }
}
