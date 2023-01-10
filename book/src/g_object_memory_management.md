# Memory Management

A GObject (or [`glib::Object`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.Object.html) in Rust terms) is a reference-counted, mutable object.
Let's see in a set of real life examples which consequences this has.

```rust ,no_run,compile_fail
#use gtk::prelude::*;
#use gtk::{self, Application, ApplicationWindow, Button, Orientation};
#
#const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";
#
fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(application: &Application) {
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

    // A mutable integer
    let mut number = 0;

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(|_| number += 1);
    button_decrease.connect_clicked(|_| number -= 1);

    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
```

Here we would like to create a simple app with two buttons.
If we click on one button, an integer number should be increased. If we press the other one, it should be decreased.
The Rust compiler refuses to compile it though.

For once the borrow checker kicked in:

```console
error[E0499]: cannot borrow `number` as mutable more than once at a time
  --> main.rs:27:37
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
  --> main.rs:26:37
   |
26 |     button_increase.connect_clicked(|_| number += 1);
   |                                     ^^^ ------ `number` is borrowed here
   |                                     |
   |                                     may outlive borrowed value `number`
   |
note: function requires argument type to outlive `'static`
  --> main.rs:26:5
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
But is there actually a way that both closures could take ownership of the same value?

Yes! That is exactly what the [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) type is there for.
The `Rc` counts the number of strong references created via [`Clone::clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone) and released via [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and only deallocates it when this number drops to zero.
We call every object containing a strong reference a shared owner of the value.
If we want to modify the content of our [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
we can use the [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html) type.

> The `Cell` class is only suitable for objects that implement the [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html) trait.
> For other objects, [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) is the way to go.
> You can learn more about interior mutability in this [section](https://marabos.nl/atomics/basics.html#interior-mutability) of the book _Rust Atomics and Locks_.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/1/main.rs">listings/g_object_memory_management/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/1/main.rs:callback}}
```

It is not very nice though to fill the scope with temporary variables like `number_copy`.
We can improve that by using the [`glib::clone!`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.clone.html) macro.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/2/main.rs">listings/g_object_memory_management/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/2/main.rs:callback}}
```

Just like `Rc<Cell<T>>`, GObjects are reference-counted and mutable.
Therefore, we can pass the buttons the same way to the closure as we did with `number`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/3/main.rs">listings/g_object_memory_management/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/3/main.rs:callback}}
```

If we now click on one button, the other button's label gets changed.

But whoops!
Did we forget about one annoyance of reference-counted systems?
Yes we did: [reference cycles](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html).
`button_increase` holds a strong reference to `button_decrease` and vice-versa.
A strong reference keeps the referenced value from being deallocated.
If this chain leads to a circle, none of the values in this cycle ever get deallocated.
With weak references we can break this cycle, because they don't keep their value alive but instead provide a way to retrieve a strong reference if the value is still alive.
Since we want our apps to free unneeded memory, we should use weak references for the buttons instead.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/4/main.rs">listings/g_object_memory_management/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/4/main.rs:callback}}
```

The reference cycle is broken.
Every time the button is clicked, `glib::clone` tries to upgrade the weak reference.
If we now for example click on one button and the other button is not there anymore, `upgrade` will return `None`.
Per default, it immediately returns from the closure with `()` as return value.
In case the closure expects a different return value `@default-return` can be specified.

Notice that we move `number` in the second closure.
If we had moved weak references in both closures, nothing would have kept `number` alive and the closure would have never been called.
Thinking about this, `button_increase` and `button_decrease` are also dropped at the end of the scope of `build_ui`.
Who then keeps the buttons alive?

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/4/main.rs">listings/g_object_memory_management/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/4/main.rs:box_append}}
```

When we append the buttons to the `gtk_box`, `gtk_box` keeps a strong reference to them.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_memory_management/4/main.rs">listings/g_object_memory_management/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_memory_management/4/main.rs:window_child}}
```

When we set `gtk_box` as child of `window`, `window` keeps a strong reference to it.
Until we close the `window` it keeps `gtk_box` and with it the buttons alive.
Since our application has only one window, closing it also means exiting the application.

As long as you use weak references whenever possible you will find it perfectly doable to avoid memory cycles within your application.
If that is ensured, you can rely on GTK to properly manage the memory of GObjects you pass to it.
