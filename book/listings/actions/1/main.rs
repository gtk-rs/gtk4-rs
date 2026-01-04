use gio::ActionEntry;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, gio, glib};

// ANCHOR: main
const APP_ID: &str = "org.gtk_rs.Actions1";

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
    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);

    // Present window
    window.present();
}
// ANCHOR: build_ui
