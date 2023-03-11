# Rust GTK 4 bindings

The project website is [here](https://gtk-rs.org/).

Rust bindings of [GTK 4](http://www.gtk.org), part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

This library contains safe Rust bindings for [GTK 4](http://www.gtk.org), a
multi-platform GUI toolkit. It is a part of [gtk-rs](http://gtk-rs.org/).

Most of this documentation is generated from the C API.
Until all parts of the documentation have been reviewed there will be incongruities
with the actual Rust API.

For a gentle introduction to gtk-rs we recommend the online book
[*GUI development with Rust and GTK 4*](https://gtk-rs.org/gtk4-rs/stable/latest/book/).

See also:

 - [gtk-rs project overview](https://gtk-rs.org)
 - [General `GLib` family types and object system overview](mod@glib)
 - [GTK documentation](https://www.gtk.org/docs/)
 - [GTK Visual Index](https://docs.gtk.org/gtk4/visual_index.html)

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.64.0`.

## "Hello, World!" example program

GTK needs to be initialized before use by calling [`init`][`fn@init`]. Creating an
[`Application`][`struct@Application`] will call [`init`][`fn@init`] for you.

The [`gtk4`](mod@crate) crate is usually renamed to `gtk`. You can find an example in
the [features](#features) section for how to do this globally in your `Cargo.toml`.

```rust,no_run
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        // Show the window.
        window.show();
    });

    app.run()
}
```

## The main loop

In a typical GTK application you set up the UI, assign signal handlers
and run the main event loop.

```rust,no_run
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.show();
    });

    application.run()
}
```

## Threads

GTK is not thread-safe. Accordingly, none of this crate's structs implement
[`Send`] or [`Sync`].

The thread where [`init`][`fn@init`] was called is considered the main thread. OS X has
its own notion of the main thread and [`init`][`fn@init`] must be called on that thread.
After successful initialization, calling any [`gtk`](mod@crate) or [`gdk`][`mod@gdk`]
functions (including [`init`][`fn@init`]) from other threads will `panic`.

Any thread can schedule a closure to be run by the main loop on the main
thread via [`glib::idle_add`][`fn@glib::idle_add`] or
[`glib::timeout_add`][`fn@glib::timeout_add`]. While
working with GTK you might need the [`glib::idle_add_local`][`fn@glib::idle_add_local`]
or [`glib::timeout_add_local`][`fn@glib::timeout_add_local`] version without the
[`Send`] bound. Those may only be called from the main thread.

# Panics

The [`gtk`](mod@crate) and [`gdk`][`mod@gdk`] crates have some run-time safety and contract
checks.

- Any constructor or free function will panic if called before [`init`][`fn@init`] or on
a non-main thread.

- Any [`&str`] or [`&Path`](std::path::Path) parameter with an interior null (`\0`) character will
cause a panic.

- Some functions will panic if supplied out-of-range integer parameters. All
such cases will be documented individually but they are not yet.

- A panic in a closure that handles signals or in any other closure passed
to a [`gtk`](mod@crate) function will abort the process.

## Features

### Library versions

By default this crate provides only GTK 4.0 APIs. You can access additional
functionality by selecting one of the `v4_2`, `v4_4`, etc. features.

`Cargo.toml` example:

```toml
[dependencies.gtk]
package = "gtk4"
version = "0.x.y"
features = ["v4_2"]
```

Take care when choosing the version to target: some of your users might
not have easy access to the latest ones. The higher the version, the fewer
users will have it installed.

## Documentation

- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4)/[Development](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/)
- Book [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/book)/[Development](https://gtk-rs.org/gtk4-rs/git/book)
- [Examples](https://github.com/gtk-rs/gtk4-rs/tree/master/examples)
- [The C API](https://docs.gtk.org/gtk4/)
- [GTK Installation Instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#library-versions).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk = {version = "0.1", package = "gdk4"}
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

### Features

| Feature | Description |
| ---     | ----------- |
| `v4_10` | Enable the new APIs part of GTK 4.10 |
| `v4_8` | Enable the new APIs part of GTK 4.8 |
| `v4_6` | Enable the new APIs part of GTK 4.6 |
| `v4_4` | Enable the new APIs part of GTK 4.4 |
| `v4_2` | Enable the new APIs part of GTK 4.2 |
| `gnome_43` | Enable all version feature flags of this crate and its dependencies to match the GNOME 43 SDK |
| `gnome_42` | Enable all version feature flags of this crate and its dependencies to match the GNOME 42 SDK |
| `unsafe-assume-initialized` | Disable checks that gtk is initialized, for use in C ABI libraries |
| `xml_validation` | Enable `xml_validation` feature of gtk4-macros |
| `blueprint` | Enable `blueprint` feature of gtk4-macros |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gdk4](https://crates.io/crates/gdk4)
- [gsk4](https://crates.io/crates/gsk4)

## License

The Rust bindings of __gtk4__ are available under the MIT License, please refer to it.
