# The main event loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and the scheduling responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="https://developer.gnome.org/glib/stable/mainloop-states.gif" /></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
You can even spawn [async functions](http://gtk-rs.org/docs/glib/struct.MainContext.html#method.spawn_local) on it.
It does all of that within the same thread.
Quickly iterating between all tasks gives the illusion of parallelism.
That is why you can move the window at the same time as a progress bar is growing.
But you surely saw GUIs that froze, at least for a few seconds.
That happens when a single task takes too long.
Let us give you one example.

<span class="filename">Filename: src/main.rs</span>


```rust ,no-run
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button};

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

    // Create a button
    let button = Button::with_label("Run stuff");

    // Connect callback
    button.connect_clicked(move |_| {
        // GUI is blocked for 10 seconds after the button is pressed
        let ten_seconds = std::time::Duration::from_secs(10);
        std::thread::sleep(ten_seconds);
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
```

After we press the button, the GUI is completely frozen.
We can't even move the window.
You probably won't force the thread to sleep within your callback,
but it is not unusual that you want to run a slightly longer operation after an event is triggered.
For that we just need to spawn a new thread and let the operation run there.

```rust ,no-run
# use gtk::prelude::*;
# use gtk::{self, Application, ApplicationWindow, Button};
# 
# fn main() {
#     // Create a new application
#     let app = Application::new(Some("org.gnome.gitlab.booktests.Devel"), Default::default())
#         .expect("Initialization failed...");
#     app.connect_activate(|app| on_activate(app));
#     // Run the application
#     app.run(&std::env::args().collect::<Vec<_>>());
# }
# 
# // When the application is launched…
# fn on_activate(application: &Application) {
#     // … create a new window …
#     let window = ApplicationWindow::new(application);
# 
#     // Create a button
#     let button = Button::with_label("Run stuff");
# 
    // Connect callback
    button.connect_clicked(move |_| {
        // The long running operation works now in a separate thread
        std::thread::spawn(move || {
            let ten_seconds = std::time::Duration::from_secs(10);
            std::thread::sleep(ten_seconds);
        });
    });
# 
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```

If you come from another language than Rust, you might be uncomfortable with the thought of spawning new threads before testing other workarounds.
Luckily Rust's safety guarantees allow you to stop worrying about the nasty bugs concurrency tends to bring.