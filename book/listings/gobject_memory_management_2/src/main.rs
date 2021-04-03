use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindowBuilder, ButtonBuilder, Orientation};
use gtk::{glib, Application};
use std::{cell::RefCell, env::args, rc::Rc};
fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(on_activate);

    // Get command-line arguments
    let args: Vec<String> = args().collect();
    // Run the application
    app.run(&args);
}
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("My GTK App")
        .build();

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
