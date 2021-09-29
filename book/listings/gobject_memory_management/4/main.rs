use std::{cell::Cell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Button, Orientation};
use gtk::{glib, Application};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a window
    // ANCHOR: window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();
    // ANCHOR_END: window

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

    // ANCHOR: callback
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@weak number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));
    // ANCHOR_END: callback

    // Add buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    // ANCHOR: set_child
    window.set_child(Some(&gtk_box));
    // ANCHOR_END: set_child
    // ANCHOR: box_append
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    // ANCHOR_END: box_append
    window.present();
}
