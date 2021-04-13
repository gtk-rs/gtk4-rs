use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclass;

mod imp {
    use super::*;
    use gtk::{glib::translate::ToGlib, subclass::prelude::*};

    #[derive(Debug)]
    pub struct CustomOrientable {
        first_label: RefCell<Option<gtk::Widget>>,
        second_label: RefCell<Option<gtk::Widget>>,
        orientation: RefCell<gtk::Orientation>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomOrientable {
        const NAME: &'static str = "ExCustomOrientable";
        type Type = super::CustomOrientable;
        type ParentType = gtk::Widget;
        type Interfaces = (gtk::Orientable,);

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk::BoxLayout>();
        }

        fn new() -> Self {
            // Here we set the default orientation.
            Self {
                first_label: RefCell::new(None),
                second_label: RefCell::new(None),
                orientation: RefCell::new(gtk::Orientation::Horizontal),
            }
        }
    }

    impl ObjectImpl for CustomOrientable {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Create the children labels.
            let first_label = gtk::Label::new(Some("Hello"));
            let second_label = gtk::Label::new(Some("World!"));
            let layout_manager = obj
                .layout_manager()
                .unwrap()
                .downcast::<gtk::BoxLayout>()
                .unwrap();
            layout_manager.set_spacing(6);
            first_label.set_parent(obj);
            second_label.set_parent(obj);
            self.first_label
                .replace(Some(first_label.upcast::<gtk::Widget>()));
            self.second_label
                .replace(Some(second_label.upcast::<gtk::Widget>()));
        }

        fn dispose(&self, _obj: &Self::Type) {
            // Child widgets need to be manually unparented in `dispose()`.
            if let Some(child) = self.first_label.borrow_mut().take() {
                child.unparent();
            }

            if let Some(child) = self.second_label.borrow_mut().take() {
                child.unparent();
            }
        }

        // Every widget that implements Orientable has to define a "orientation"
        // property like below, gtk::Orientation::Horizontal is a placeholder
        // for the initial value.
        //
        // glib::ParamFlags::CONSTRUCT allows us to set that property the moment
        // we create a new instance of the widget
        fn properties() -> &'static [glib::ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpec::enum_(
                    "orientation",
                    "orientation",
                    "Orientation",
                    gtk::Orientation::static_type(),
                    gtk::Orientation::Horizontal.to_glib(),
                    glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "orientation" => {
                    let orientation = value.get().unwrap().unwrap();
                    self.orientation.replace(orientation);
                    // We have to set the value in our layout manager as well.
                    let layout_manager = obj
                        .layout_manager()
                        .unwrap()
                        .downcast::<gtk::BoxLayout>()
                        .unwrap();
                    layout_manager.set_orientation(orientation);
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            pspec: &glib::ParamSpec,
        ) -> glib::Value {
            match pspec.name() {
                "orientation" => self.orientation.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for CustomOrientable {}
    impl OrientableImpl for CustomOrientable {}
}

glib::wrapper! {
    pub struct CustomOrientable(ObjectSubclass<imp::CustomOrientable>)
        @extends gtk::Widget, @implements gtk::Orientable;
}

impl Default for CustomOrientable {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomOrientable {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create CustomOrientable")
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.orientable_subclass"),
        Default::default(),
    );
    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let bx = gtk::Box::new(gtk::Orientation::Vertical, 6);
        let orientable = CustomOrientable::new();
        let button = gtk::Button::with_label("Switch orientation");

        button.connect_clicked(glib::clone!(@weak orientable => move |_| {
            match orientable.orientation() {
                gtk::Orientation::Horizontal => orientable.set_orientation(gtk::Orientation::Vertical),
                gtk::Orientation::Vertical => orientable.set_orientation(gtk::Orientation::Horizontal),
                _ => unreachable!(),
            };
        }));

        orientable.set_halign(gtk::Align::Center);
        bx.append(&orientable);
        bx.append(&button);
        bx.set_margin_top(18);
        bx.set_margin_bottom(18);
        bx.set_margin_start(18);
        bx.set_margin_end(18);

        window.set_child(Some(&bx));
        window.show();
    });

    application.run();
}
