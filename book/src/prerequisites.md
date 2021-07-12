# Prerequisites

There are a few recommended ways to set up your workstation in order to develop `gtk-rs` applications.
Let us go through them one by one.

## Cargo

[Cargo](https://doc.rust-lang.org/cargo/index.html) is Rust's build system and package manager.
If following the book is all you care about, using only Cargo will work fine for you.

Let us begin by installing all necessary tools.
First follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).

Now create a new project by executing:
```bash
cargo new my-gtk-app
```

Add the following lines to your dependencies in `Cargo.toml`.
```toml
gtk = { version = "0.1", package = "gtk4" }
```

Now you can run your application by executing:
```bash
cargo run
```

## Cargo + Meson

Cargo is *almost* enough, but it is not well suited for handling resources such as icons or UI definition files.
That is why we recommend to use [Meson](https://mesonbuild.com/) on top of it.
It is cross-platform and its syntax is very readable.
Meson takes care of
- translations
- building and installing [resources](resources.html)
- installing auxiliary files such as icons and [settings schemas](settings.html)

Here as well, you first follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).
Finally, install Meson by following the instructions on the [Meson website](https://mesonbuild.com/Getting-meson.html).

You can download a ready-to-use gtk-rust-template [here](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template).
Follow the instructions in the README to initialize your own application.
Then configure your project.
```bash
meson --prefix=/usr build
```

In order to compile and install it run the following commands.
You have to execute it every time you modify your application.
```bash
ninja -C build && ninja -C build install
```

Now the application should be in a folder included in your system path.
You can either start it with the application launcher of your choice or in the terminal.

## Cargo + Meson + Flatpak

If you develop on Linux, using Flatpak is the most convenient option.
With Flatpak your whole workflow is containerized and your users get the very same application you develop on (including all dependencies). 
First, assure that Flatpak is installed on your system, check this [website](https://flatpak.org/setup/) to see if any steps are necessary on your distribution.
Then download the [gtk-rust-template](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template) and follow the instructions in its README.

Then either install:

- [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder) or
- [VSCodium](https://flathub.org/apps/details/com.vscodium.codium) together with the [rust-analyzer](https://open-vsx.org/extension/matklad/rust-analyzer) and [flatpak](https://open-vsx.org/extension/bilelmoussaoui/flatpak-vscode) extensions.

That is it.
The build dependencies can be downloaded by the IDE.
With GNOME Builder, you only have to press the run button for that.
