# GObjects and their memory management

As mentioned in the section before, GObject is the base class all widgets inherit of.
That means all GObjects share a set of common features.
For now we will will focus on their memory management.
GObjects are reference-counted, mutable objects, so they behave very similar to `Rc<RefCell<T>>`.
Let's see in a real life example why this is something we want to have.
Here we have an integer variable, which gets increased by pressing one button and decreased when pressing the other one.

# Example 1 - borrow error
```rust , compile_fail
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Button, Orientation};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create two buttons
    let button_increase = Button::with_label("Increase");
    let button_decrease = Button::with_label("Decrease");

    let mut number = 0;

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(|_| number += 1);
    button_decrease.connect_clicked(|_| number -= 1);

    // Add buttons
    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
```

# Example 2 - fix
```rust , no_run
use std::{cell::RefCell, rc::Rc};

use gtk::prelude::*;
use gtk::Application;
use gtk::{self, ApplicationWindow, Box, Button, Orientation};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
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

    // Reference-counted object with inner-mutability
    let number = Rc::new(RefCell::new(0));

    // Connect callbacks, when a button is clicked `number` will be changed
    let number_copy_1 = number.clone();
    let number_copy_2 = number.clone();
    button_increase.connect_clicked(move |_| *number_copy_1.borrow_mut() += 1);
    button_decrease.connect_clicked(move |_| *number_copy_2.borrow_mut() -= 1);

    // Add buttons
    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
```

# Example 3 - introduce clone!
```rust , no_run
use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Box, Button, Orientation};
use gtk::{glib, Application};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
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
    button_increase.connect_clicked(clone!(@strong number => move |_| {
        *number.borrow_mut() += 1;
    }));
    button_decrease.connect_clicked(clone!(@strong number => move |_| {
        *number.borrow_mut() -= 1;
    }));

    // Add buttons
    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
```


# Example 4 - use clone! with widget
```rust , no_run
use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Box, Button, Orientation};
use gtk::{glib, Application};

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    let button_increase = Button::with_label("Increase");
    let button_decrease = Button::with_label("Decrease");

    let number = Rc::new(RefCell::new(0));

    button_increase.connect_clicked(clone!(@strong number, @strong button_decrease => move |_| {
        *number.borrow_mut() += 1;
        button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong number, @strong button_increase => move |_| {
        *number.borrow_mut() -= 1;
        button_increase.set_label(&number.borrow().to_string());
    }));

    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
```



# Example 5 - fix memory cycle
```rust , no_run
use std::{cell::RefCell, rc::Rc};

use glib::clone;
use gtk::prelude::*;
use gtk::{self, ApplicationWindow, Box, Button, Orientation};
use gtk::{glib, Application};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
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
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@strong number, @weak button_decrease => move |_| {
        *number.borrow_mut() += 1;
        button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong number, @weak button_increase => move |_| {
        *number.borrow_mut() -= 1;
        button_increase.set_label(&number.borrow().to_string());
    }));

    // Add buttons
    let gtk_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
```