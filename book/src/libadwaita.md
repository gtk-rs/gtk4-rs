# Libadwaita

If you target a certain platform with your GUI, you will want to follow the platform's [Human Interface Guidelines](https://en.wikipedia.org/wiki/Human_interface_guidelines) (HIG).
With a GTK application, chances are the platform is either [elementary OS](https://elementary.io) or [GNOME](https://www.gnome.org/).
In this chapter we will discuss how to follow GNOME's [HIG](https://developer.gnome.org/hig/) with [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/). 

Libadwaita is a GTK 4 library which:
- provides widgets to better follow GNOME's HIG
- provides widgets to let application [adapt their UI](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/adaptive-layouts.html) between desktop and mobile devices
- integrates the Adwaita stylesheet
- allows runtime recoloring with [named colors](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/named-colors.html)
- adds [API](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.StyleManager.html) to support the cross-desktop dark style preference

In order to use the Rust bindings add the following line to your dependencies in `Cargo.toml`, where `X.X` should be replaced with the most up-to-date version of the [libadwaita crate](https://crates.io/crates/libadwaita).

```toml
adw = { version = "X.X", package = "libadwaita" }
```

The versions of the `gtk4` and `libadwaita` crates need to be synced.
Just remember that when you update one of them to the newest version to update the other one as well. 

Installation of the library itself works similar to GTK.
Just follow the installation instruction that is suitable for your distribution.

## Linux

Fedora and derivatives:

```
sudo dnf install libadwaita-devel
```

Debian and derivatives:

```
sudo apt install libadwaita-1-dev
```

Arch and derivatives:

```
sudo pacman -S libadwaita
```

Nix:  
Add the following inputs to your derivation:

```
libadwaita
```

## macOS

```
brew install libadwaita
```

## Windows

From the Windows start menu, search for `x64 Native Tools Command Prompt for VS 2019`.
That will open a terminal configured to use MSVC x64 tools.
From there, run the following commands:

```
cd /
git clone https://gitlab.gnome.org/GNOME/libadwaita.git --depth 1
cd libadwaita
meson setup builddir -Dprefix=C:/gnome -Dintrospection=disabled -Dvapi=false -Dexamples=false
meson install -C builddir
```
