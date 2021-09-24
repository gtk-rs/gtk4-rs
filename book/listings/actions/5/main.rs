use gio::SimpleAction;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow, Button, Label, Orientation};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    let label = Label::builder().label(&format!("Counter: {}", 0)).build();

    // Add action "count" to `window` which takes no parameters
    let action_quit =
        SimpleAction::new_stateful("count", Some(&i32::static_variant_type()), &0.to_variant());
    action_quit.connect_activate(clone!(@weak label => move |action, parameter| {
        // Get state
        let mut state = action
        .state()
        .expect("Could not get state.")
        .get::<i32>()
        .expect("The value needs to be of type `i32`.");

        // Get parameter
        let parameter = parameter
            .expect("Could not get parameter.")
            .get::<i32>()
            .expect("The value needs to be of type `i32`.");

        // Increase state by parameter and store state
        state += parameter;
        action.set_state(&state.to_variant());

        // Update label with new state
        label.set_label(&format!("Counter: {}", state));
    }));

    window.add_action(&action_quit);

    // ANCHOR: button_builder
    // Create a button with label and action
    let button = Button::builder()
        .label("Press me!")
        .action_name("win.count")
        .action_target(&1.to_variant())
        .build();
    // ANCHOR_END: button_builder

    // Create `gtk_box` and add `button` and `label` to it
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    // Add button
    window.set_child(Some(&gtk_box));

    // Present window to the user
    window.present();
}
// ANCHOR: build_ui
