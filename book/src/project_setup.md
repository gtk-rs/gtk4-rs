# Project Setup

Let's begin by installing all necessary tools.
First, follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).

Now, create a new project by executing:
```
cargo new my-gtk-app
```

Find out the GTK 4 version on your machine by running

```
pkg-config --modversion gtk4
```

Use this information to add the [gtk4 crate](https://crates.io/crates/gtk4) to your dependencies in `Cargo.toml`.
At the time of this writing the newest version is `4.8`.

```
cargo add gtk4 --rename gtk --features v4_8
```

Now, you can run your application by executing:
```
cargo run
```
