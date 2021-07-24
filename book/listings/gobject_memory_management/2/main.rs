use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Button, Orientation};
use gtk::{glib, Application};
use std::{cell::Cell, rc::Rc};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk.example")
        .build();

    // Connect to "activate" signal
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
fn build_ui(app: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

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

    // Reference-counted object with inner mutability
    let number = Rc::new(Cell::new(0));
    // Connect callbacks
    // When a button is clicked, `number` will be changed
    // ANCHOR: callback
    button_increase.connect_clicked(clone!(@strong number => move |_| {
        number.set(number.get() + 1);
    }));
    button_decrease.connect_clicked(move |_| {
        number.set(number.get() - 1);
    });
    // ANCHOR_END: callback

    // Add buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
