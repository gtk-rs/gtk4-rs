mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindowBuilder};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
// ANCHOR: activate
fn build_ui(application: &Application) {
    // Create a window
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
            // args[0] would return the `CustomButton` instance
            let number = args[1]
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");
            println!("The maximum number {} has been reached", number);
            None
        })
        .unwrap();
    // ANCHOR_END: signal_handling

    window.set_child(Some(&button));
    window.present();
}
// ANCHOR_END: activate
