# Memory Management

Memory management when writing a gtk-rs app can be a bit tricky.
Let's have a look why that is the case and how to deal with that.

With our first example, we have window with a single button.
Every button click should increment an integer `number` by one.

```rust ,no_run,compile_fail
#use gtk::prelude::*;
#use gtk::{self, glib, Application, ApplicationWindow, Button};
#
#const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";
#
// DOES NOT COMPILE!
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
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

    // A mutable integer
    let mut number = 0;

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(|_| number += 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&button_increase)
        .build();

    // Present the window
    window.present();
}
```

The Rust compiler refuses to compile this application while spitting out multiple error messages.
Let's have a look at them one by one.

```console

error[E0373]: closure may outlive the current function, but it borrows `number`, which is owned by the current function
   |
32 |     button_increase.connect_clicked(|_| number += 1);
   |                                     ^^^ ------ `number` is borrowed here
   |                                     |
   |                                     may outlive borrowed value `number`
   |
note: function requires argument type to outlive `'static`
   |
32 |     button_increase.connect_clicked(|_| number += 1);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `number` (and any other referenced variables), use the `move` keyword
   |
32 |     button_increase.connect_clicked(move |_| number += 1);
   |  
```

Our closure only borrows `number`.
Signal handlers in GTK require `'static` lifetimes for their references, so we cannot borrow a variable that only lives for the scope of the function `build_ui`.
The compiler also suggests how to fix this.
By adding the `move` keyword in front of the closure, `number` will be moved into the closure.

```rust ,no_run,compile_fail
#use gtk::prelude::*;
#use gtk::{self, glib, Application, ApplicationWindow, Button};
#
#const APP_ID: &str = "org.gtk_rs.GObjectMemoryManagement0";
#
#fn main() -> glib::ExitCode {
#    // Create a new application
#    let app = Application::builder().application_id(APP_ID).build();
#
#    // Connect to "activate" signal of `app`
#    app.connect_activate(build_ui);
#
#    // Run the application
#    app.run()
#}
#
#fn build_ui(application: &Application) {
#    // Create two buttons
#    let button_increase = Button::builder()
#        .label("Increase")
#        .margin_top(12)
#        .margin_bottom(12)
#        .margin_start(12)
#        .margin_end(12)
#        .build();
#
    // DOES NOT COMPILE!
    // A mutable integer
    let mut number = 0;

    // Connect callbacks
    // When a button is clicked, `number` should be changed
    button_increase.connect_clicked(move |_| number += 1);
#
#    // Create a window
#    let window = ApplicationWindow::builder()
#        .application(application)
#        .title("My GTK App")
#        .child(&button_increase)
#        .build();
#
#    // Present the window
#    window.present();
#}
```

This still leaves the following error message:

```console

error[E0594]: cannot assign to `number`, as it is a captured variable in a `Fn` closure
   |
32 |     button_increase.connect_clicked(move |_| number += 1);
   |                                              ^^^^^^^^^^^ cannot assign
```

In order to understand that error message we have to understand the difference between the three closure traits `FnOnce`, `FnMut` and `Fn`.
APIs that take closures implementing the `FnOnce` trait give the most freedom to the API consumer.
The closure is called only once, so it can even consume its state.
Signal handlers can be called multiple times, so they cannot accept `FnOnce`.

The more restrictive `FnMut` trait doesn't allow closures to consume their state, but they can still mutate it.
Signal handlers can't allow this either, because they can be called from inside themselves.
This would lead to multiple mutable references which the borrow checker doesn't appreciate at all.

This leaves `Fn`.
State can be immutably borrowed, but then how can we modify `number`?
We need a data type with interior mutability like [`std::cell::Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html).

> The `Cell` class is only suitable for objects that implement the [`Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html) trait.
> For other objects, [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) is the way to go.
> You can learn more about interior mutability in this [section](https://marabos.nl/atomics/basics.html#interior-mutability) of the book _Rust Atomics and Locks_.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/2/main.rs">listings/g_object_memory_management/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/1/main.rs:build_ui}}
```

This now compiles as expected.
Let's try a slightly more complicated example: two buttons which both modify the same `number`.
For that, we need a way that both closures take ownership of the same value?

That is exactly what the [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) type is there for.
`Rc` counts the number of strong references created via [`Clone::clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone) and released via [`Drop::drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop), and only deallocates the value when this number drops to zero.
If we want to modify the content of our [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
we can again use the [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html) type.


Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/2/main.rs">listings/g_object_memory_management/2/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/2/main.rs:callback}}
```

It is not very nice though to fill the scope with temporary variables like `number_copy`.
We can improve that by using the [`glib::clone!`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.clone.html) macro.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/3/main.rs">listings/g_object_memory_management/3/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/3/main.rs:callback}}
```

Just like `Rc<Cell<T>>`, GObjects are reference-counted and mutable.
Therefore, we can pass the buttons the same way to the closure as we did with `number`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/4/main.rs">listings/g_object_memory_management/4/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/4/main.rs:callback}}
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

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/5/main.rs">listings/g_object_memory_management/5/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/5/main.rs:callback}}
```

The reference cycle is broken.
Every time the button is clicked, `glib::clone` tries to upgrade the weak reference.
If we now for example click on one button and the other button is not there anymore, the callback will be skipped.
Per default, it immediately returns from the closure with `()` as return value.
In case the closure expects a different return value `@default-return` can be specified.

Notice that we move `number` in the second closure.
If we had moved weak references in both closures, nothing would have kept `number` alive and the closure would have never been called.
Thinking about this, `button_increase` and `button_decrease` are also dropped at the end of the scope of `build_ui`.
Who then keeps the buttons alive?

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/5/main.rs">listings/g_object_memory_management/5/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/5/main.rs:box_append}}
```

When we append the buttons to the `gtk_box`, `gtk_box` keeps a strong reference to them.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/g_object_memory_management/5/main.rs">listings/g_object_memory_management/5/main.rs</a>

```rust
{{#rustdoc_include ../listings/g_object_memory_management/5/main.rs:window_child}}
```

When we set `gtk_box` as child of `window`, `window` keeps a strong reference to it.
Until we close the `window` it keeps `gtk_box` and with it the buttons alive.
Since our application has only one window, closing it also means exiting the application.

As long as you use weak references whenever possible, you will find it perfectly doable to avoid memory cycles within your application.
Without memory cycles, you can rely on GTK to properly manage the memory of GObjects you pass to it.
