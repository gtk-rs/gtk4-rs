use std::cell::Cell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{self, glib, Application, ApplicationWindow, Button, Orientation};

const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}
fn build_ui(app: &Application) {
    // Create two buttons
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // ANCHOR: callback
    // Reference-counted object with inner-mutability
    let number = Rc::new(Cell::new(0));

    // Connect callbacks, when a button is clicked `number` will be changed
    let number_copy = number.clone();
    button_increase.connect_clicked(move |_| number_copy.set(number_copy.get() + 1));
    button_decrease.connect_clicked(move |_| number.set(number.get() - 1));
    // ANCHOR_END: callback

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
