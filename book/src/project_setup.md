# Project Setup

There are two recommended ways to set up your workstation in order to develop `gtk-rs` applications.
Let us go through them one by one.

## Cargo

[Cargo](https://doc.rust-lang.org/cargo/index.html) is Rust's build system and package manager.
If following the book is all you care about, using only Cargo will work fine for you.

Let us begin by installing all necessary tools.
First, follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).

Now, create a new project by executing:
```bash
cargo new my-gtk-app
```

Add the following lines to your dependencies in `Cargo.toml`, where `X.X` should be replaced with the most up-to-date version of the [gtk4 crate](https://crates.io/crates/gtk4).

```toml
gtk = { version = "X.X", package = "gtk4" }
```

>Per default `gtk4-rs` is compatible with all GTK 4 releases.
>If you want to use functionality that has been added to later releases, you have to specify this as a feature.
>If you want to use functionality of GTK 4.6, you would add the following to your gtk dependency in `Cargo.toml`.
> 
> ```toml
>gtk = { version = "X.X", package = "gtk4", features = ["v4_6"]}
>```
>This will only work if your available GTK version is indeed >= 4.6.
>You can get the version by executing the following command:
>```
>pkg-config --modversion gtk4
>```

Now, you can run your application by executing:
```bash
cargo run
```

## Cargo + Meson

Cargo is *almost* enough, but it is not well suited for handling resources such as icons or UI definition files.
That is why we recommend to use [Meson](https://mesonbuild.com/) on top of it.
It is cross-platform, and its syntax is very readable.
Meson takes care of
- translations,
- building and installing [resources](resources.html) as well as
- installing auxiliary files such as icons and [settings schemas](settings.html).

Here as well, you first follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).
Finally, install Meson by following the instructions on the [Meson website](https://mesonbuild.com/Getting-meson.html).

You can download a ready-to-use gtk-rust-template [here](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template).
Follow the instructions in the README to initialize your own application.
Then configure your project.
```bash
meson setup builddir
```

In order to compile and install it run the following command.
You have to execute it every time you modify your application.
```bash
meson install -C builddir
```

Now, the application should be in a folder included in your system path.
You can either start it with the application launcher of your choice or from within your terminal.

