# Memory management of GObjects

We start by learning about the memory management of GObjects.
GObjects are reference-counted, mutable objects, so they behave very similar to `Rc<RefCell<T>>`.
Let us see in a set of real life examples why this is something we want to have.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run,compile_fail
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Box, Button, Orientation};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
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

    // A mutable integer
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
For once the borrow checker kicked in:
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
The compiler cannot know when the user presses a button, so references must live forever.
And our `number` gets immediately deallocated after it reaches the end of its scope.
The error message is also suggesting that we could take ownership of `number`... But is there actually a way that both closures could take ownership of the same object?

Yes! That is exactly what the [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) type is there for.
With multiple owners we have to move the borrow check from compile time to run time, 
and we also want to be able to update the content of our [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html).
For that we can use the [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) type.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_1/src/main.rs:callback}}

```

It not very nice though to fill the scope with temporary variables like `number_copy_1`.
We can improve that by using the `glib::clone!` macro.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_2/src/main.rs:callback}}
```

Since GObjects are reference-counted and mutable as well, so we can pass the buttons the same way as we did with `number`.
If we now click on one button, the other button's label gets changed.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_3/src/main.rs:callback}}
```
Expand the code, copy and try it on your own machine.
It will work fine.

But whoops!
Didn't we forget about one annoyance of reference-counted systems?
Of course we did: reference cycles.
`button_increase` holds a strong reference to `button_decrease` and vice-versa.
A strong reference keeps the referenced object from being deallocated.
If this chain leads to a circle, none of the objects in this cycle ever get deallocated.
We obviously do not want our apps to keep allocating memory, so let us use weak references for the buttons instead.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_4/src/main.rs:callback}}
```

The reference cycle is broken.
If we now click on one button and the other button is not there anymore, the closure will simply not be get called.
Most of the time, this is the behavior you will want.
Notice however that we kept the strong reference to `number`.
If we had only used weak references, `number` would have been dropped and the closure would have never been called.
Unlike with tricky-to-debug reference cycles, you would have immediately noticed that after testing your app.
That is why we recommend to default to weak references.
