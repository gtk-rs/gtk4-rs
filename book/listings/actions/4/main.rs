use gio::ActionEntry;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Button, Label, Orientation, gio, glib,
};

const APP_ID: &str = "org.gtk_rs.Actions4";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    let original_state = 0;
    let label = Label::builder()
        .label(format!("Counter: {original_state}"))
        .build();
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
        .halign(Align::Center)
        .build();
    gtk_box.append(&button);
    gtk_box.append(&label);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .child(&gtk_box)
        .build();

    // Add action "count" to `window` taking an integer as parameter
    let action_count = ActionEntry::builder("count")
        .parameter_type(Some(&i32::static_variant_type()))
        .state(original_state.to_variant())
        .activate(move |_, action, parameter| {
            // Get state
            let mut state = action
                .state()
                .expect("Could not get state.")
                .get::<i32>()
                .expect("The variant needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The variant needs to be of type `i32`.");

            // Increase state by parameter and store state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {state}"));
        })
        .build();
    window.add_action_entries([action_count]);

    // Present window
    window.present();
}
// ANCHOR: build_ui
