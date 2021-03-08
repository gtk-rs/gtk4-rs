# Hello World!

Now that we have got a working installation, let us get right into it!

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
```
We create an `Application` instance, with an application id and the default application flags.
[This guide](https://wiki.gnome.org/HowDoI/ChooseApplicationID) helps you find a suitable application id for your app.

We execute `cargo run` in order to build and run it.
The terminal is blocked, but nothing appears on ours screen.
GTK warns us though, that it would have expected that something would be called in its `activate` step.
So let us create a window there.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

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
    // … create a new window …
    let window = ApplicationWindow::new(application);
    window.present();
}
```
That's better!

<div style="text-align:center"><img src="images/hello_world_empty.png" /></div>

Normally we expect to be able to interact with the user interface.
Also, the name of the chapter suggests that the phrase “Hello World!” will be involved.

<span class="filename">Filename: src/main.rs</span>


```rust ,no_run
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

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
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create a button
    let button = Button::with_label("Run stuff");

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
```
There is now a button and if we click on it, its label becomes “Hello World!”.

<div style="text-align:center"><img src="images/hello_world_button.png" /></div>

Was not that hard to create our first `gtk-rs` app, right?
Let us now get a better understanding of what we did here.
