//! # Custom GtkEditable
//!
//! This example shows how to create a custom gtk::Editable which is the equivalent
//! of creating a custom text entry that can have tags shown on it.
//! It's a copy of the tagged entry demo from gtk4-demo

use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, glib};
use std::env::args;

static CSS: &str = "
tag {
    margin: 4px 0px;
    padding: 4px;
    border-radius: 4px;
    background: lightskyblue;
  }
  tag box {
    border-spacing: 4px;
  }
  tag label,
  tag image {
    color: white;
  }
  tag button {
    min-height: 0;
    min-width: 0;
    padding: 0;
    border: 1px solid white;
  }
  
  entry.tagged {
    border-spacing: 4px;
  }
";

mod imp {
    use super::*;
    use glib::subclass::Signal;
    use glib::ParamSpec;
    use once_cell::sync::Lazy;
    use std::cell::{Cell, RefCell};

    pub struct CustomEditable {
        text: gtk::Text,
        pub spinner: RefCell<Option<gtk::Spinner>>,
        pub show_spinner: Cell<bool>,
    }

    impl Default for CustomEditable {
        fn default() -> Self {
            Self {
                text: gtk::Text::new(),
                spinner: RefCell::default(),
                show_spinner: Cell::new(false),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomEditable {
        const NAME: &'static str = "CustomEditable";
        type Type = super::CustomEditable;
        type ParentType = gtk::Widget;
        type Interfaces = (gtk::Editable,);

        fn class_init(klass: &mut Self::Class) {
            // The call to `gtk_editable_install_properties` at `class_init` is automatically done for you.
            klass.set_layout_manager_type::<gtk::BoxLayout>();
            klass.set_css_name("entry");
            klass.set_accessible_role(gtk::AccessibleRole::TextBox);
        }
    }

    impl ObjectImpl for CustomEditable {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpec::boolean(
                    "show-spinner",
                    "Spinner shown",
                    "Whether the editable has a visible spinner",
                    false,
                    glib::ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            editable: &Self::Type,
            id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            // In case this is a property that's automatically added for Editable implementations.
            if !self.delegate_set_property(editable, id, value, pspec) {
                match pspec.get_name() {
                    "show-spinner" => {
                        editable.set_show_spinner(value.get_some().unwrap());
                    }
                    _ => unimplemented!(),
                }
            }
        }

        fn get_property(
            &self,
            editable: &Self::Type,
            id: usize,
            pspec: &glib::ParamSpec,
        ) -> glib::Value {
            // In case this is a property that's automatically added for Editable implementations.
            if let Some(value) = self.delegate_get_property(editable, id, pspec) {
                value
            } else {
                match pspec.get_name() {
                    "show-spinner" => self.show_spinner.get().to_value(),
                    _ => unimplemented!(),
                }
            }
        }

        fn constructed(&self, editable: &Self::Type) {
            // Most of the times when implementing Editable, you just want to embed something like
            // `gtk::Text` inside a more complex widget. In such case, your implementation most forward the `gtk::Text`
            // or any `Editable` to the delegate. That starts by returning at `EditableImpl::get_delegate`.
            //
            // In such case, the user has to initialize the delegate at `constructed` and stop it at `dispose`.
            // It mostly consists of wiring up/down some signals from the delegate (`gtk::Text` in this case) to the internal editable
            // implementation.
            editable.init_delegate();
            self.text.set_hexpand(true);
            self.text.set_vexpand(true);

            self.text.set_parent(editable);
            editable.add_css_class("tagged");
            editable.set_enable_undo(true);
        }

        fn dispose(&self, editable: &Self::Type) {
            // Wire down the delegate signals machinery
            editable.finish_delegate();
            self.text.unparent();
            while let Some(child) = editable.get_first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for CustomEditable {
        fn grab_focus(&self, _widget: &Self::Type) -> bool {
            self.text.grab_focus()
        }
    }
    impl EditableImpl for CustomEditable {
        fn get_delegate(&self, _editable: &Self::Type) -> Option<gtk::Editable> {
            Some(self.text.clone().upcast())
        }
    }

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
    pub struct CustomEditable(ObjectSubclass<imp::CustomEditable>) @extends gtk::Widget, @implements gtk::Editable;
}

impl CustomEditable {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a CustomEditable")
    }

    pub fn add_tag(&self, tag: &CustomTag) {
        tag.set_parent(self);
    }

    pub fn remove_tag(&self, tag: &CustomTag) {
        tag.unparent();
    }

    pub fn set_show_spinner(&self, show_spinner: bool) {
        let self_ = imp::CustomEditable::from_instance(self);
        if self_.show_spinner.get() == show_spinner {
            return;
        }

        if show_spinner {
            let spinner = gtk::SpinnerBuilder::new()
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .build();
            spinner.start();

            spinner.set_parent(self);
            self_.spinner.replace(Some(spinner));
        } else {
            if let Some(spinner) = self_.spinner.borrow_mut().take() {
                spinner.unparent();
            }
        }
        self_.show_spinner.set(show_spinner);
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
        } else {
            if let Some(button) = self_.button.borrow_mut().take() {
                self_.container.remove(&button);
            }
        }
        self_.has_close_button.set(has_close_button);
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Custom Editable"));
    window.set_default_size(500, 500);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 12);
    container.set_valign(gtk::Align::Center);
    container.set_halign(gtk::Align::Center);

    let editable = CustomEditable::new();
    editable.set_halign(gtk::Align::Fill);

    container.append(&editable);

    let horizontal_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    horizontal_container.set_halign(gtk::Align::Center);

    let add_tag_button = gtk::Button::with_label("Add Tag");
    add_tag_button.set_halign(gtk::Align::Center);
    add_tag_button.connect_clicked(clone!(@weak editable => move |_btn| {
        let tag = CustomTag::new("Blue");
        tag.connect_local_id(imp::CustomTag::signals()[0].signal_id(), None, false, clone!(@weak editable, @weak tag => @default-return None, move |_args| {
            editable.remove_tag(&tag);
            None
        })).unwrap();
        tag.connect_local_id(imp::CustomTag::signals()[1].signal_id(), None, false, move |_args| {
            println!("Tag clicked");
            None
        }).unwrap();
        editable.add_tag(&tag);
    }));
    horizontal_container.append(&add_tag_button);

    let show_spinner = gtk::CheckButtonBuilder::new()
        .halign(gtk::Align::End)
        .label("Show Spinner")
        .build();
    show_spinner
        .bind_property("active", &editable, "show-spinner")
        .build();
    horizontal_container.append(&show_spinner);

    container.append(&horizontal_container);
    window.set_child(Some(&container));
    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.editable"),
        Default::default(),
    )
    .expect("Initialization failed...");

    let provider = gtk::CssProvider::new();
    provider.load_from_data(CSS.as_bytes());
    gtk::StyleContext::add_provider_for_display(
        &gdk::Display::get_default().unwrap(),
        &provider,
        800,
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
