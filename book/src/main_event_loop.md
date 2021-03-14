# The main event loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and scheduling the responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="img/main_event_loop.png" /></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
It does all of that within the same thread.
Quickly iterating between all tasks gives the illusion of parallelism.
That is why you can move the window at the same time as a progress bar is growing.


However, you surely saw GUIs that became unresponsive, at least for a few seconds.
That happens when a single task takes too long.
Let us look at one example.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_1/src/main.rs}}
```

After we press the button, the GUI is completely frozen for 10 seconds.
We can't even move the window.
The `sleep` call is an artificial example,
but it is not unusual wanting to run a slightly longer operation in one go.
For that we just need to spawn a new thread and let the operation run there.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_2/src/main.rs:callback}}
```

If you come from another language than Rust, you might be uncomfortable with the thought of spawning new threads before even looking at other options.
Luckily, Rust's safety guarantees allow you to stop worrying about the nasty bugs, concurrency tends to bring.

Normally we want to keep track of the work in the thread.
In our case, we don't want the user to spawn additional threads while an existing one is still running.
In order to achieve that we can create a channel.
The main loop allows us to send a message from multiple places to a single receiver at the main thread.
We want to send a `bool` to inform, whether we want the button to react to clicks or not.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_3/src/main.rs:callback}}
```

Spawning threads is not the only way to run operations asynchronously.
You can also let the main loop take care of running `async` functions.
If you do that from the main thread use [`spawn_local`](http://gtk-rs.org/docs/glib/struct.MainContext.html#method.spawn_local), from other threads [`spawn`](http://gtk-rs.org/docs/glib/struct.MainContext.html#method.spawn) has to be used.
The converted code looks and behaves very similar to the multi-threaded code.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_4/src/main.rs:callback}}
```

Since we are single-threaded again, we could even get rid of the channels while achieving the same result.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_5/src/main.rs:callback}}
```

But why didn't we do the same thing with our multi-threaded example?

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run,compile_fail
# use glib::{clone, MainContext, PRIORITY_DEFAULT};
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindowBuilder, ButtonBuilder};
# 
# fn main() {
#     // Create a new application
#     let app = Application::new(Some("org.gtk.example"), Default::default())
#         .expect("Initialization failed...");
#     app.connect_activate(|app| on_activate(app));
# 
#     // Get command-line arguments
#     let args: Vec<String> = std::env::args().collect();
#     // Run the application
#     app.run(&args);
# }
# 
# // When the application is launched…
# fn on_activate(application: &Application) {
#     // … create a new window …
#     let window = ApplicationWindowBuilder::new()
#         .application(application)
#         .title("My GTK App")
#         .build();
# 
#     // Create a button
#     let button = ButtonBuilder::new()
#         .label("Press me!")
#         .margin_top(12)
#         .margin_bottom(12)
#         .margin_start(12)
#         .margin_end(12)
#         .build();
# 
    // DOES NOT COMPILE
    
    // Connect callback
    button.connect_clicked(move |button| {
        button.clone();
        // The long running operation runs now in a separate thread
        std::thread::spawn(move || {
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            let ten_seconds = std::time::Duration::from_secs(10);
            std::thread::sleep(ten_seconds);
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

After reference cycles we found the second disadvantage of GTK GObjects: they are not thread safe.

So when should you spawn an `async` block and when should you spawn a thread?
- If you have `async` functions for your IO-bound operations at your disposal, feel free to spawn them on the main loop.
- If your operation is computation-bound or there is no `async` function available, you have to spawn threads.

