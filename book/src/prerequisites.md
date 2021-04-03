# Installation

Let us begin by installing all necessary tools.
First follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).

Now create a new project by executing:
```bash
cargo new my-gtk-app
```

Add the following lines to your `Cargo.toml` and you are ready to go!
```toml
[dependencies.gtk]
version = "0.1"
package = "gtk4"
```

# Buildsystem

TODO
