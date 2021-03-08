use gtk::prelude::*;
use gtk::Application;
use gtk::{self, ApplicationWindow, Button, Orientation};
use std::{cell::RefCell, rc::Rc};
fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window
    let window = ApplicationWindow::new(application);

    // Create two buttons
    let button_increase = Button::with_label("Increase");
    let button_decrease = Button::with_label("Decrease");

    // ANCHOR: callback
    // Reference-counted object with inner-mutability
    let number = Rc::new(RefCell::new(0));

    // Connect callbacks, when a button is clicked `number` will be changed
    let number_copy_1 = number.clone();
    button_increase.connect_clicked(move |_| *number_copy_1.borrow_mut() += 1);
    button_decrease.connect_clicked(move |_| *number.borrow_mut() -= 1);
    // ANCHOR_END: callback

    // Add buttons
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
