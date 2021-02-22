# Memory management of GObjects

As mentioned in the section before, GObject is the base class all widgets inherit of.
That means all GObjects share a set of common features.
Within this section, we will focus on their memory management.
GObjects are reference-counted, mutable objects, so they behave very similar to `Rc<RefCell<T>>`.
Let's see in a real life example why this is something we want to have.

<span class="filename">Filename: src/main.rs</span>

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
The idea of this code is that we have a simple app with two buttons.
If we click on one button, an integer number gets increased, if we press the other one, it gets decreased.
The Rust compiler refuses to compile it though.
For once the famous borrow checker kicked in:
```console
error[E0499]: cannot borrow `number` as mutable more than once at a time
  --> src/main.rs:27:37
   |
26 |     button_increase.connect_clicked(|_| number += 1);
   |     ------------------------------------------------
   |     |                               |   |
   |     |                               |   first borrow occurs due to use of `number` in closure
   |     |                               first mutable borrow occurs here
   |     argument requires that `number` is borrowed for `'static`
27 |     button_decrease.connect_clicked(|_| number -= 1);
   |                                     ^^^ ------ second borrow occurs due to use of `number` in closure
   |                                     |
   |                                     second mutable borrow occurs here
```
Also, the compiler tells us that our closures may outlive `number`:
```console

error[E0373]: closure may outlive the current function, but it borrows `number`, which is owned by the current function
  --> src/main.rs:26:37
   |
26 |     button_increase.connect_clicked(|_| number += 1);
   |                                     ^^^ ------ `number` is borrowed here
   |                                     |
   |                                     may outlive borrowed value `number`
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:26:5
   |
26 |     button_increase.connect_clicked(|_| number += 1);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `number` (and any other referenced variables), use the `move` keyword
   |
26 |     button_increase.connect_clicked(move |_| number += 1);
   |                                     ^^^^^^^^
```
Thinking about the second error message, it makes sense that the closure requires the lifetimes of references to be `'static`.
The compiler can't know when the user presses a button, so references must live forever.
Our `number` gets immediately deallocated though after it reaches the end of its scope.
The error message is also suggesting, that we could take ownership of `number`, but is there a way that both closures can take ownership of the same object?
Yes! That's exactly what [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) is there for.
We still have to move the borrow check from compile to run time.
And that's what [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) is there for.

<span class="filename">Filename: src/main.rs</span>

```rust , no_run
#use std::{cell::RefCell, rc::Rc};
#
#use gtk::prelude::*;
#use gtk::Application;
#use gtk::{self, ApplicationWindow, Box, Button, Orientation};
#
#fn main() {
#    // Create a new application
#    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
#        .expect("Initialization failed...");
#    app.connect_activate(|app| on_activate(app));
#    // Run the application
#    app.run(&std::env::args().collect::<Vec<_>>());
#}
#
#// When the application is launched…
#fn on_activate(application: &Application) {
#    // … create a new window
#    let window = ApplicationWindow::new(application);
#
#    // Create two buttons
#    let button_increase = Button::with_label("Increase");
#    let button_decrease = Button::with_label("Decrease");
#
    // Reference-counted object with inner-mutability
    let number = Rc::new(RefCell::new(0));

    // Connect callbacks, when a button is clicked `number` will be changed
    let number_copy_1 = number.clone();
    let number_copy_2 = number.clone();
    button_increase.connect_clicked(move |_| *number_copy_1.borrow_mut() += 1);
    button_decrease.connect_clicked(move |_| *number_copy_2.borrow_mut() -= 1);
#
#    // Add buttons
#    let gtk_box = Box::new(Orientation::Vertical, 0);
#    window.set_child(Some(&gtk_box));
#    gtk_box.append(&button_increase);
#    gtk_box.append(&button_decrease);
#    window.present();
#}
```

It not very nice though to spam the scope with temporary variables like `number_copy_1` and `number_copy_2`.
We can improve it by using the `glib::clone!` macro.

<span class="filename">Filename: src/main.rs</span>

```rust , no_run
#use std::{cell::RefCell, rc::Rc};
#
#use glib::clone;
#use gtk::prelude::*;
#use gtk::{self, ApplicationWindow, Box, Button, Orientation};
#use gtk::{glib, Application};
#
#fn main() {
#    // Create a new application
#    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
#        .expect("Initialization failed...");
#    app.connect_activate(|app| on_activate(app));
#    // Run the application
#    app.run(&std::env::args().collect::<Vec<_>>());
#}
#
#// When the application is launched…
#fn on_activate(application: &Application) {
#    // … create a new window …
#    let window = ApplicationWindow::new(application);
#
#    // Create two buttons
#    let button_increase = Button::with_label("Increase");
#    let button_decrease = Button::with_label("Decrease");
#
#    // Reference-counted object with inner mutability
#    let number = Rc::new(RefCell::new(0));
#
#    // Connect callbacks
#    // When a button is clicked, `number` will be changed
    button_increase.connect_clicked(clone!(@strong number => move |_| {
        *number.borrow_mut() += 1;
    }));
    button_decrease.connect_clicked(clone!(@strong number => move |_| {
        *number.borrow_mut() -= 1;
    }));
#
#    // Add buttons
#    let gtk_box = Box::new(Orientation::Vertical, 0);
#    window.set_child(Some(&gtk_box));
#    gtk_box.append(&button_increase);
#    gtk_box.append(&button_decrease);
#    window.present();
#}
```

Since GObjects are reference-counted and mutable, we can even pass the corresponding buttons to the closures.
If we now click on one button, the other button's label gets changed.

<span class="filename">Filename: src/main.rs</span>

```rust , no_run
#use std::{cell::RefCell, rc::Rc};
#
#use glib::clone;
#use gtk::prelude::*;
#use gtk::{self, ApplicationWindow, Box, Button, Orientation};
#use gtk::{glib, Application};
#
#// When the application is launched…
#fn on_activate(application: &Application) {
#    // … create a new window …
#    let window = ApplicationWindow::new(application);
#
#    let button_increase = Button::with_label("Increase");
#    let button_decrease = Button::with_label("Decrease");
#
#    let number = Rc::new(RefCell::new(0));

    button_increase.connect_clicked(clone!(@strong number, @strong button_decrease => 
        move |_| {
            *number.borrow_mut() += 1;
            button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong number, @strong button_increase => 
        move |_| {
            *number.borrow_mut() -= 1;
            button_increase.set_label(&number.borrow().to_string());
    }));
#
#    let gtk_box = Box::new(Orientation::Vertical, 0);
#    window.set_child(Some(&gtk_box));
#    gtk_box.append(&button_increase);
#    gtk_box.append(&button_decrease);
#    window.present();
#}
#
#fn main() {
#    // Create a new application
#    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
#        .expect("Initialization failed...");
#    app.connect_activate(|app| on_activate(app));
#    // Run the application
#    app.run(&std::env::args().collect::<Vec<_>>());
#}
```
Expand the code, copy and try it on your own machine.
It will work fine.
But whoops!
Didn't we forget about one annoyance of reference-counted systems?
Of course we did: reference cycles.
`button_increase` holds a strong reference to `button_decrease` and vice-versa.
A strong reference keeps the referenced object from being deallocated.
If this leads to a circle, none of the objects in this cycle ever get deallocated.
We don't want our apps to senselessly allocating memory, so let's use weak references for the buttons instead.

<span class="filename">Filename: src/main.rs</span>

```rust , no_run
#use std::{cell::RefCell, rc::Rc};
#
#use glib::clone;
#use gtk::prelude::*;
#use gtk::{self, ApplicationWindow, Box, Button, Orientation};
#use gtk::{glib, Application};
#
#fn main() {
#    // Create a new application
#    let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
#        .expect("Initialization failed...");
#    app.connect_activate(|app| on_activate(app));
#    // Run the application
#    app.run(&std::env::args().collect::<Vec<_>>());
#}
#
#// When the application is launched…
#fn on_activate(application: &Application) {
#    // … create a new window …
#    let window = ApplicationWindow::new(application);
#
#    // Create two buttons
#    let button_increase = Button::with_label("Increase");
#    let button_decrease = Button::with_label("Decrease");
#
#    // Reference-counted object with inner mutability
#    let number = Rc::new(RefCell::new(0));

    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(@strong number, @weak button_decrease => 
        move |_| {
            *number.borrow_mut() += 1;
            button_decrease.set_label(&number.borrow().to_string());
    }));
    button_decrease.connect_clicked(clone!(@strong number, @weak button_increase =>
        move |_| {
            *number.borrow_mut() -= 1;
            button_increase.set_label(&number.borrow().to_string());
    }));
#
#    // Add buttons
#    let gtk_box = Box::new(Orientation::Vertical, 0);
#    window.set_child(Some(&gtk_box));
#    gtk_box.append(&button_increase);
#    gtk_box.append(&button_decrease);
#    window.present();
#}
```

The reference cycle is broken.
If we now click on one button and the other button is not there anymore, the closure simply does not get called.
Most of the time, this is the behavior you will want.
Notice however that we kept the strong reference to `number`.
If we had only used weak references, `number` would have been dropped and the closure would have never been called.
Unlike with tricky-to-debug memory cycles, you would have immediately noticed that after testing your app.
That is why we recommend to default to weak references.