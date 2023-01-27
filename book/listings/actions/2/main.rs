use gio::SimpleAction;
use glib::clone;
use gtk::gio::SimpleActionGroup;
use gtk::prelude::*;
use gtk::{gio, glib, Application, ApplicationWindow};

// ANCHOR: main
const APP_ID: &str = "org.gtk_rs.Actions2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.close".
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);

    // Run the application
    app.run()
}
// ANCHOR_END: main

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(360)
        .build();

    // Add action "close" to `window` taking no parameter
    let action_close = SimpleAction::new("close", None);
    action_close.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_close);

    // ANCHOR: action_group
    // Create a new action group and add actions to it
    let actions = SimpleActionGroup::new();
    window.insert_action_group("win", Some(&actions));
    actions.add_action(&action_close);
    // ANCHOR_END: action_group

    // Present window
    window.present();
}
// ANCHOR: build_ui
