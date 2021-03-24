//! This example shows how to create a custom widget that behaves like `gtk::Button`.

use std::cell::RefCell;
use std::env;

use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;
    use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct Button {
        /// Reference to the child widget.
        ///
        /// In our case it's a text label for the button. Since this example only uses a
        /// `gtk::Label`, the type could've been `Option<gtk::Label>`. However, a real button might
        /// switch between a label widget and an icon widget, and in general your widget can contain
        /// arbitrary children. Thus we used `Option<gtk::Widget>` to show how to handle any widget
        /// and to make the example easier to tweak and play with.
        ///
        /// Widgets automatically store strong references to their children, added in `set_parent()`
        /// and removed in `unparent()`. Therefore, this field could be a `WeakRef<gtk::Widget>`.
        /// Using a strong reference is just a little clearer.
        child: RefCell<Option<gtk::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Button {
        const NAME: &'static str = "ExButton";
        type Type = super::Button;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk::BinLayout>();

            // Make it look like a GTK button.
            klass.set_css_name("button");
        }
    }

    impl ObjectImpl for Button {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Create the child label.
            let child = gtk::Label::new(Some("Hello world!"));
            child.set_parent(obj);
            *self.child.borrow_mut() = Some(child.upcast::<gtk::Widget>());

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
            // Child widgets need to be manually unparented in `dispose()`.
            if let Some(child) = self.child.borrow_mut().take() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for Button {}
}

glib::wrapper! {
    pub struct Button(ObjectSubclass<imp::Button>)
        @extends gtk::Widget;
}

impl Button {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
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
