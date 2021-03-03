# Installation

Let's begin by installing all necessary tools.
First follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) to install GTK 4.

Then install Rust with the `rustup` tool
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Now create a new project with
```bash
cargo new my-gtk-project
```

Add the following lines to your `Cargo.toml` and you are ready to go!
```toml
[dependencies.gtk]
version = "0.1"
package = "gtk4"
```
