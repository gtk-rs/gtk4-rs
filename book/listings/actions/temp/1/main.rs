use gio::SimpleAction;
use glib::clone;
use gtk::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, ApplicationWindow, Button};

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

    // Add action "quit" to `window` which takes no parameters
    let action_quit = SimpleAction::new("quit", None);
    action_quit.connect_activate(clone!(@weak window => move |_, _| {
        window.close();
    }));
    window.add_action(&action_quit);

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Activate "win.quit" without passing parameters
        button.activate_action("win.quit", None);
    });

    // Add button
    window.set_child(Some(&button));

    // Present window to the user
    window.present();
}
// ANCHOR: build_ui
