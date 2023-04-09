mod custom_button;

use custom_button::CustomButton;
use glib::BindingFlags;
use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectProperties4";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
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
        .transform_to(|_, number: i32| {
            let incremented_number = number + 1;
            Some(incremented_number.to_value())
        })
        // How to transform "number" from `button_2` to "number" of `button_1`
        .transform_from(|_, number: i32| {
            let decremented_number = number - 1;
            Some(decremented_number.to_value())
        })
        .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
        .build();
    // ANCHOR_END: bind_numbers

    // ANCHOR: connect_notify
    // The closure will be called
    // whenever the property "number" of `button_1` gets changed
    button_1.connect_number_notify(|button| {
        println!("The current number of `button_1` is {}.", button.number());
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

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
