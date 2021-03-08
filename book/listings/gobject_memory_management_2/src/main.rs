use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Button, Orientation};
use gtk::{glib, Application};
use std::{cell::RefCell, rc::Rc};
fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Run the application
    app.run(&args);
}
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);
    // Create two buttons
    let button_increase = Button::with_label("Increase");
    let button_decrease = Button::with_label("Decrease");
    // Reference-counted object with inner mutability
    let number = Rc::new(RefCell::new(0));
    // Connect callbacks
    // When a button is clicked, `number` will be changed
    // ANCHOR: callback
    button_increase.connect_clicked(clone!(@strong number => move |_| {
        *number.borrow_mut() += 1;
    }));
    button_decrease.connect_clicked(move |_| {
        *number.borrow_mut() -= 1;
    });
    // ANCHOR_END: callback

    // Add buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
