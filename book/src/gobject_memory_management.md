# Memory management of GObjects

GObjects are reference-counted, mutable objects, so they behave very similar to `Rc<RefCell<T>>`.
Let us see in a set of real life examples which consequences this has.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run,compile_fail
use std::env::args;

use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button, Orientation};

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
    let gtk_box = gtk::Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&gtk_box));
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);
    window.present();
}
```
Here we would like to create a simple app with two buttons.
If we click on one button, an integer number should be increased. If we press the other one, it should be decreased.
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
The error message is also suggesting that we could take ownership of `number`.
But is there actually a way that both closures could take ownership of the same object?

Yes! That is exactly what the [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html) type is there for.
With `Rc` your object becomes reference counted.
The runtime counts the number of strong references to the object and only deallocates it when this number drops to zero.
We call every object containing a strong reference a shared owner of the object.
With multiple owners we have to move the borrow check from compile time to run time. 
If we then want to modify the content of our [Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html),
we can use the [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html) type.
`RefCell` then checks Rust's borrow rules during runtime, namely that there can only be one mutable borrow at a time or multiple immutable borrows. `RefCell` panics if these rules are violated.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_1/src/main.rs:callback}}
```

It is not very nice though to fill the scope with temporary variables like `number_copy_1`.
We can improve that by using the `glib::clone!` macro.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_2/src/main.rs:callback}}
```

Just like `Rc<RefCell<T>>`, GObjects are reference-counted and mutable.
Therefore, we can pass the buttons the same way to the closure as we did with `number`.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_3/src/main.rs:callback}}
```
If we now click on one button, the other button's label gets changed.

But whoops!
Did we forget about one annoyance of reference-counted systems?
Yes we did: reference cycles.
`button_increase` holds a strong reference to `button_decrease` and vice-versa.
A strong reference keeps the referenced object from being deallocated.
If this chain leads to a circle, none of the objects in this cycle ever get deallocated.
With weak references we can break this cycle, because they do not keep their object alive but instead provide a way to retrieve a strong reference if the object is still alive.
Since we want our apps to free unneeded memory, we should use weak references for the buttons instead[^1].

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_4/src/main.rs:callback}}
```

The reference cycle is broken.
If we now click on one button and the other button is not there anymore, the closure will simply not be called.
Notice however that we kept the *strong* reference to `number`.
If we had a *weak* reference, no one would have kept `number` alive and the closure would have never been called.

Thinking about this, `button_increase` and `button_decrease` are also dropped at the end of the scope of `on_activate`.
Who then keeps the buttons alive?

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_4/src/main.rs:box_append}}
```

When we append the buttons to the `gtk_box`, `gtk_box` keeps a strong reference to them.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_4/src/main.rs:set_child}}
```

When we set `gtk_box` as child of `window`, `window` keeps a strong reference to it.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_memory_management_4/src/main.rs:window}}
```

During the creation of our `window`, we pass `application` to it.
Because of that, `application` holds a strong reference to `window`.
`application` lives for the whole lifetime of our program, which explains why weak references within our closures are sufficient.

As long as you use weak references whenever possible you will find it perfectly doable to avoid memory cycles within your application.
Then, you can fully rely on GTK to properly manage the memory of GObjects you pass to it.

[^1]: In this simple example, GTK actually resolves the reference cycle on its own once you close the window.
However, the general point to avoid strong references whenever possible remains valid.
