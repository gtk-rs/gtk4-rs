use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindowBuilder, ButtonBuilder, Orientation};
use gtk::{glib, Application};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(on_activate);

    // Run the application
    app.run();
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    // ANCHOR: window
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();
    // ANCHOR_END: window

    // Create two buttons
    let button_increase = ButtonBuilder::new()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = ButtonBuilder::new()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Reference-counted object with inner mutability
    let number = Rc::new(RefCell::new(0));

    // ANCHOR: callback
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@strong number, @weak button_decrease =>
        move |_| {
            *number.borrow_mut() += 1;
            button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@weak button_increase =>
        move |_| {
            *number.borrow_mut() -= 1;
            button_increase.set_label(&number.borrow().to_string());
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
