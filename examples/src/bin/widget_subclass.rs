use std::cell::RefCell;
use std::env;

use gio::prelude::*;
use glib::glib_wrapper;
use gtk::prelude::*;

mod imp {
    use super::*;
    use glib::{glib_object_subclass, WeakRef};
    use gtk::subclass::prelude::*;

    #[derive(Debug)]
    pub struct Button {
        child: RefCell<WeakRef<gtk::Widget>>,
    }

    impl ObjectSubclass for Button {
        const NAME: &'static str = "ExButton";
        type Type = super::Button;
        type ParentType = gtk::Widget;
        type Instance = glib::subclass::simple::InstanceStruct<Self>;
        type Class = glib::subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk::BinLayout>();

            // Make it look like a GTK button.
            klass.set_css_name("button");
        }

        fn new() -> Self {
            Self {
                child: RefCell::new(WeakRef::new()),
            }
        }
    }

    impl ObjectImpl for Button {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Create the child label.
            let child = gtk::Label::new(Some("Hello world!"));
            child.set_parent(obj);
            *self.child.borrow_mut() = child.upcast::<gtk::Widget>().downgrade();

            // Make it look like a GTK button with a label (as opposed to an icon).
            obj.add_css_class("text-button");

            // Connect a gesture to handle clicks.
            let gesture = gtk::GestureClick::new();
            gesture.connect_released(|gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                println!("Button pressed!");
            });
            obj.add_controller(&gesture);
        }

        fn dispose(&self, _obj: &Self::Type) {
            // Child widgets need to be manually unparented.
            if let Some(child) = self.child.borrow().upgrade() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for Button {}
}

glib_wrapper! {
    pub struct Button(ObjectSubclass<imp::Button>)
        @extends gtk::Widget;
}

impl Button {
    pub fn new() -> Self {
        glib::Object::new(Self::static_type(), &[])
            .unwrap()
            .downcast()
            .unwrap()
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.widget_subclass"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);
        let button = Button::new();
        button.set_margin_top(18);
        button.set_margin_bottom(18);
        button.set_margin_start(18);
        button.set_margin_end(18);
        window.set_child(Some(&button));
        window.show();
    });

    application.run(&env::args().collect::<Vec<_>>());
}
