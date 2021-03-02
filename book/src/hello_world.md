# Hello World!

https://wiki.gnome.org/HowDoI/ChooseApplicationID


```rust ,no_run
use gtk::prelude::*;
use gtk::Application;

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
        .expect("Initialization failed...");
    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}
```

```rust ,no_run
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
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


```rust ,no_run
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default())
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
        // Set the label to "Hello World!" after the button is pressed
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
```