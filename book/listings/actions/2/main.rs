use gio::ActionEntry;
use glib::clone;
use gtk::gio::SimpleActionGroup;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.Actions2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);
    // ANCHOR: accel
    // Set keyboard accelerator to trigger "custom-group.close".
    app.set_accels_for_action("custom-group.close", &["<Ctrl>W"]);
    // ANCHOR_END: accel

    // Run the application
    app.run()
}

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .build();

    // Add action "close" to `window` taking no parameter
    let action_close = ActionEntry::builder("close")
        .activate(clone!(#[weak] window , move |_, _, _| {
            window.close();
        }))
        .build();

    // ANCHOR: action_group
    // Create a new action group and add actions to it
    let actions = SimpleActionGroup::new();
    actions.add_action_entries([action_close]);
    window.insert_action_group("custom-group", Some(&actions));
    // ANCHOR_END: action_group

    // Present window
    window.present();
}
// ANCHOR: build_ui
