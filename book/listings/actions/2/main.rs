use gio::SimpleAction;
use glib::clone;
use gtk::gio::SimpleActionGroup;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow};

// ANCHOR: main
fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Set keyboard accelerator to trigger "win.quit".
    app.set_accels_for_action("win.quit", &["<primary>Q"]);

    // Run the application
    app.run();
}
// ANCHOR_END: main

// ANCHOR: build_ui
fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Add action "quit" to `window` which takes no parameter
    let action_quit = SimpleAction::new("quit", None);
    action_quit.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_quit);

    // ANCHOR: action_group
    // Create a new action group and add actions to it
    let actions = SimpleActionGroup::new();
    window.insert_action_group("win", Some(&actions));
    actions.add_action(&action_quit);
    // ANCHOR_END: action_group

    // Present window to the user
    window.present();
}
// ANCHOR: build_ui
