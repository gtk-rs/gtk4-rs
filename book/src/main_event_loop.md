# The Main Event Loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and scheduling the responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="img/main_event_loop.png" alt="Diagram showing the main event loop"/></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
It does all of that within the same thread.
Quickly iterating between all tasks gives the illusion of parallelism.
That is why you can move the window at the same time as a progress bar is growing.


However, you surely saw GUIs that became unresponsive, at least for a few seconds.
That happens when a single task takes too long.
Let's look at one example.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/main_event_loop/1/main.rs">listings/main_event_loop/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/main_event_loop/1/main.rs}}
```

After we press the button, the GUI is completely frozen for five seconds.
We can't even move the window.
The `sleep` call is an artificial example,
but it is not unusual wanting to run a slightly longer operation in one go.

## How to Avoid Blocking the Main Loop

In order to avoid blocking the main loop we can spawn a new thread and let the operation run there.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/main_event_loop/2/main.rs">listings/main_event_loop/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/main_event_loop/2/main.rs:callback}}
```

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/main_event_loop_2.webm" type="video/webm">
  <p>A video which shows that after pressing the button, the window can still be moved</p>
 </video>
</div>


> If you come from another language than Rust, you might be uncomfortable with the thought of spawning new threads before even looking at other options.
> Luckily, Rust's safety guarantees allow you to stop worrying about the nasty bugs that concurrency tends to bring.



Normally we want to keep track of the work in the thread.
In our case, we don't want the user to spawn additional threads while an existing one is still running.
In order to achieve that we can create a channel.
The main loop allows us to send a message from multiple places to a single receiver at the main thread.
We want to send a `bool` to inform, whether we want the button to react to clicks or not.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/main_event_loop/3/main.rs">listings/main_event_loop/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/main_event_loop/3/main.rs:callback}}
```

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/main_event_loop_3.webm" type="video/webm">
  <p>The button now stops being responsive for 10 seconds after being pressed</p>
 </video>
</div>


> Per default, [`glib::clone!`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.clone.html) returns `()` when upgrading of a weak reference fails.
> [`glib::Receiver::attach`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.Receiver.html#method.attach) expects a closure with a return value of type [`glib::ControlFlow`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/enum.ControlFlow.html).
> This is why we specify `@default-return` as `glib::ControlFlow::Break` to clarify that the closure not be called anymore as soon as the upgrade of a weak reference fails.


Spawning threads is not the only way to run operations asynchronously.
You can also let the main loop take care of running [`async`](https://rust-lang.github.io/async-book/) functions.
If you do that from the main thread use [`spawn_local`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.MainContext.html#method.spawn_local), from other threads [`spawn`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.MainContext.html#method.spawn) has to be used.
The converted code looks and behaves very similar to the multi-threaded code.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/main_event_loop/4/main.rs">listings/main_event_loop/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/main_event_loop/4/main.rs:callback}}
```

Since we are single-threaded again, we could even get rid of the channels while achieving the same result.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/main_event_loop/5/main.rs">listings/main_event_loop/5/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/main_event_loop/5/main.rs:callback}}
```

But why did we not do the same thing with our multi-threaded example?

```rust ,no_run,compile_fail
# use std::{thread, time::Duration};
# 
# use glib::{clone, MainContext, PRIORITY_DEFAULT};
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow, Button};
# 
# fn main() {
#     // Create a new application
#     let app = Application::builder()
#        .application_id("org.gtk_rs.MainEventLoop6")
#        .build();
#
#     // Connect to "activate" signal
#     app.connect_activate(build_ui);
# 
#     // Get command-line arguments
#     let args: Vec<String> = args().collect();
#     // Run the application
#     app.run(&args);
# }
# 
# // When the application is launched…
# fn build_ui(application: &Application) {
#     // Create a window
#     let window = ApplicationWindow::builder()
#         .application(application)
#         .title("My GTK App")
#         .build();
# 
#     // Create a button
#     let button = Button::builder()
#         .label("Press me!")
#         .margin_top(12)
#         .margin_bottom(12)
#         .margin_start(12)
#         .margin_end(12)
#         .build();
# 
    // DOES NOT COMPILE
    
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        button.clone();
        // The long running operation runs now in a separate thread
        thread::spawn(move || {
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            // Activate the button again
            button.set_sensitive(true);
        });
    });
# 
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```

Simply because we would get this error message:

```console
error[E0277]: `NonNull<GObject>` cannot be shared between threads safely

help: within `gtk4::Button`, the trait `Sync` is not implemented for `NonNull<GObject>`
```

After reference cycles we found the second disadvantage of GTK GObjects: They are not thread safe.

So when should you spawn an `async` block and when should you spawn a thread?
- If you have `async` functions for your IO-bound operations at your disposal, feel free to spawn them on the main loop.
- If your operation is computation-bound or there is no `async` function available, you have to spawn threads.
