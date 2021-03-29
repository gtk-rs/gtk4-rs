use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindowBuilder};
use std::{cell::RefCell, env::args};

// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary objects and traits for subclassing
    use glib::subclass::Signal;
    use gtk::subclass::prelude::*;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(Default)]
    pub struct CustomButton {
        number: RefCell<i32>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "MyGtkAppCustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
    }
    // ANCHOR: object_impl
    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_label(&self.number.borrow().to_string());
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder(
                    // Signal name
                    "max-number-reached",
                    // Types of the values which will be sent to the receiver
                    &[i32::static_type().into()],
                    // Type of the value the receiver sends back
                    <()>::static_type().into(),
                )
                .build()]
            });
            SIGNALS.as_ref()
        }
    }
    // ANCHOR_END: object_impl

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // ANCHOR: button_impl
    static MAX_NUMBER: i32 = 8;
    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {
        fn clicked(&self, button: &Self::Type) {
            // Increase `number` by 1
            *self.number.borrow_mut() += 1;
            // If `number` reached `MAX_NUMBER`,
            // emit "max-number-reached" signal and set `number` back to 0
            if *self.number.borrow() == MAX_NUMBER {
                button
                    .emit_by_name("max-number-reached", &[&*self.number.borrow()])
                    .unwrap();
                self.number.replace(0);
            }
            // Set label of `button`
            button.set_label(&self.number.borrow().to_string())
        }
    }
    // ANCHOR_END: button_impl
}

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Get command-line arguments
    let args: Vec<String> = args().collect();
    // Run the application
    app.run(&args);
}
// ANCHOR: activate
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // Create a button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // ANCHOR: signal_handling
    button
        .connect_local("max-number-reached", false, move |args| {
            // Get the number from the arguments
            // args.get(0) would return the `CustomButton` instance
            let number = args.get(1).unwrap().get::<i32>().unwrap().unwrap();
            println!("The maximum number {} has been reached", number);
            None
        })
        .unwrap();
    // ANCHOR_END: signal_handling

    window.set_child(Some(&button));
    window.present();
}
// ANCHOR_END: activate
