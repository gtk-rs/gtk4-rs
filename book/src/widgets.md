# Widgets

Widgets are the components that make up a GTK application.
GTK offers many-preexisting ones and if those do not fit you can even create custom ones (see Chapter **??**).
There are display widgets, buttons, containers and windows.
One kind of widget might be able to contain other widgets, it might present information and it might react to interaction.

The [Widget Gallery](https://developer.gnome.org/gtk4/stable/ch08.html) is useful to find out which widget fits your needs.
Let's say we want a button.
We have quite a bit of choice here, but let's take the most simple one.
Its name can be found out by clicking on the image ⇒ it is a `GtkButton`.

<div style="text-align:center"><img src="https://developer.gnome.org/gtk4/stable/button.png" /></div>


The [GTK documentation](https://developer.gnome.org/gtk3/stable/GtkButton.html) tells us also that the (simplified) inheritance tree of a `GtkButton` looks like this:

```console
 GObject
  ╰── GtkWidget
       ╰── GtkContainer
            ╰── GtkBin
                 ╰── GtkButton
```

A GtkButton is part of an inheritance tree with GObject at the top.
Additionally, the GtkButton implements the interfaces `GtkBuildable`, `GtkActionable` and `GtkActivatable`.

Now let us compare that with the [gtk-rs docs](https://gtk-rs.org/gtk4-rs/gtk4/index.html).
Gtk-rs symbols are already namespaced, so we search for `Button` instead of `GtkButton`.
The result then tells us which methods and traits it [implements](https://gtk-rs.org/gtk4-rs/gtk4/struct.Button.html#implements).
We find that every base class and interface in the GTK docs has a corresponding trait implemented.
Assuming that we want to connect a callback, we are mostly interested in its “button-behavior”.
And indeed, the trait `ButtonExt` includes the method [`connect_clicked`](https://gtk-rs.org/gtk4-rs/gtk4/trait.ButtonExt.html#tymethod.connect_clicked).

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, Button};

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

Please note that Rust requires bringing traits into scope, before using one of its methods.
In our example we did that by adding the following line:

```rust ,no_run
use gtk::prelude::*;
# 
# fn main(){}
```
With it, we import all necessary traits for dealing with widgets.
You probably want to bring the prelude into scope in most of your source files.
