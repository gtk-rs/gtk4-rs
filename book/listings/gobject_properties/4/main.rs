mod custom_button;

use custom_button::CustomButton;
use glib::BindingFlags;
use gtk::{glib, Align, Orientation};
use gtk::{prelude::*, Box};
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: buttons
    // Create the buttons
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();
    // ANCHOR_END: buttons

    // ANCHOR: bind_numbers
    // Assure that "number" of `button_2` is always 1 higher than "number" of `button_1`
    button_1
        .bind_property("number", &button_2, "number")
        // How to transform "number" from `button_1` to "number" of `button_2`
        .transform_to(|_, value| {
            let number = value
                .get::<i32>()
                .expect("The property needs to be of type `i32`.");
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        // How to transform "number" from `button_2` to "number" of `button_1`
        .transform_from(|_, value| {
            let number = value
                .get::<i32>()
                .expect("The property needs to be of type `i32`.");
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();
    // ANCHOR_END: bind_numbers

    // ANCHOR: connect_notify
    // The closure will be called whenever the property "number" of `button_1` gets changed
    button_1.connect_notify_local(Some("number"), move |button, _| {
        let number = button
            .property("number")
            .expect("The property needs to exist and be readable.")
            .get::<i32>()
            .expect("The property needs to be of type `i32`.");
        println!("The current number of `button_1` is {}.", number);
    });
    // ANCHOR_END: connect_notify

    // Set up box
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);
    window.set_child(Some(&gtk_box));
    window.present();
}
