mod custom_button;

use custom_button::CustomButton;
use glib::BindingFlags;
use gtk::{glib, Align, Orientation};
use gtk::{prelude::*, BoxBuilder};
use gtk::{Application, ApplicationWindowBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Run the application
    app.run();
}
// ANCHOR: activate
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

    // ANCHOR: buttons
    // Create the buttons
    let button_1 = CustomButton::new();
    let button_2 = CustomButton::new();
    // ANCHOR_END: buttons

    // ANCHOR: bind_number
    button_1
        .bind_property("number", &button_2, "number")
        .flags(BindingFlags::BIDIRECTIONAL)
        .build();
    // ANCHOR_END: bind_number

    // ANCHOR: bind_label
    button_1
        .bind_property("label", &button_2, "label")
        .flags(BindingFlags::BIDIRECTIONAL)
        .build();
    // ANCHOR_END: bind_label

    // ANCHOR: connect_notify

    // The closure will be called whenever the property "number" of `button_1` gets changed
    button_1.connect_notify_local(Some("number"), move |button, _| {
        let number = button
            .get_property("number")
            .unwrap()
            .get::<i32>()
            .unwrap()
            .unwrap();
        println!("The current number is {}", number);
    });
    // ANCHOR_END: connect_notify

    // Set up box
    let gtk_box = BoxBuilder::new()
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
// ANCHOR_END: activate
