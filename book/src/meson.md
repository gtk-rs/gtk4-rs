# Building with Meson

So far we have been using Cargo and `build.rs` to compile our GResources.
While this works great for development, it has limitations when distributing your app:

- Resources are embedded into the binary at compile time
- No support for system integration (desktop files, icons, GSettings schemas)
- Difficult to integrate with distribution packaging systems

[Meson](https://mesonbuild.com/) is a build system used by most GNOME projects.
It provides:

- Dynamic GResource loading from installed locations
- Automatic installation of desktop files, icons, and GSettings schemas
- Easy integration with Flatpak and distribution packaging
- Foundation for internationalization (gettext)

In this chapter we convert our To-Do app to use Meson.

## Project Structure

With Meson, we organize the project differently:

```
todo/
├── meson.build              # Root build configuration
├── meson.options            # Build options (e.g., profile)
├── Cargo.toml               # Rust dependencies (no build.rs)
├── src/
│   ├── meson.build          # Cargo integration
│   ├── config.rs.in         # Template for app metadata
│   ├── main.rs
│   └── ...
└── data/
    ├── meson.build          # Desktop file, schema
    ├── icons/
    │   └── meson.build
    └── resources/
        ├── meson.build      # GResource compilation
        └── ...
```

## Root meson.build

The root `meson.build` file defines the project and sets up common variables:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/meson.build">listings/todo/9/meson.build</a>

```meson
{{#include ../listings/todo/9/meson.build}}
```

Key points:
- `gnome = import('gnome')` provides helpers for GNOME integration
- We support `default` and `development` profiles
- Development builds get a `.Devel` suffix on the app ID
- `gnome.post_install()` updates icon caches and compiles schemas after installation

## Build Options

The `meson.options` file defines configurable options:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/meson.options">listings/todo/9/meson.options</a>

```meson
{{#include ../listings/todo/9/meson.options}}
```

## GResource Compilation

Instead of `glib_build_tools::compile_resources()` in `build.rs`, we use Meson's `gnome.compile_resources()`:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/data/resources/meson.build">listings/todo/9/data/resources/meson.build</a>

```meson
{{#include ../listings/todo/9/data/resources/meson.build}}
```

The `gresource_bundle: true` option creates a standalone `.gresource` file that gets installed to `pkgdatadir` (e.g., `/usr/share/todo/resources.gresource`).

## The config.rs.in Template

Meson generates `config.rs` from a template, substituting build-time values:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/src/config.rs.in">listings/todo/9/src/config.rs.in</a>

```rust
{{#rustdoc_include ../listings/todo/9/src/config.rs.in}}
```

The `@APP_ID@` and `@PKGDATADIR@` placeholders are replaced during configuration.

## Cargo Integration

The `src/meson.build` generates `config.rs` and invokes Cargo:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/src/meson.build">listings/todo/9/src/meson.build</a>

```meson
{{#include ../listings/todo/9/src/meson.build}}
```

The `depends: resources` ensures GResources are compiled before the Rust code.

## Loading Resources Dynamically

The main change in Rust code is how we load resources:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/src/main.rs">listings/todo/9/src/main.rs</a>

```rust
{{#rustdoc_include ../listings/todo/9/src/main.rs:resource_loading}}
```

Instead of `gio::resources_register_include!()` which embeds resources at compile time, we use `gio::Resource::load()` to load them from the installed path at runtime.

We also use `config::APP_ID` instead of a hardcoded constant.

## System Integration Files

### Desktop File

The desktop file tells the system how to display and launch your app:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/data/org.gtk_rs.Todo9.desktop.in">listings/todo/9/data/org.gtk_rs.Todo9.desktop.in</a>

```ini
{{#include ../listings/todo/9/data/org.gtk_rs.Todo9.desktop.in}}
```

The `@APP_ID@` placeholder is substituted during the build.

### GSettings Schema

The schema defines application settings:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/data/org.gtk_rs.Todo9.gschema.xml">listings/todo/9/data/org.gtk_rs.Todo9.gschema.xml</a>

```xml
{{#include ../listings/todo/9/data/org.gtk_rs.Todo9.gschema.xml}}
```

### Application Icon

The application needs an icon.

# TODO: Create a simple icon

You can create your own icons with Inkscape and [App Icon Preview](https://flathub.org/apps/org.gnome.design.AppIconPreview), which also lets you generate the development version of an icon.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/data/icons/meson.build">listings/todo/9/data/icons/meson.build</a>

```meson
{{#include ../listings/todo/9/data/icons/meson.build}}
```

### Data meson.build

This file installs the desktop file, schema, and includes subdirectories:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/todo/9/data/meson.build">listings/todo/9/data/meson.build</a>

```meson
{{#include ../listings/todo/9/data/meson.build}}
```

## Building and Running

### Development Build

```
cd listings/todo/9
meson setup builddir -Dprofile=development
meson compile -C builddir
```

To run without installing, use `meson devenv` which sets up the environment:

```
meson devenv -C builddir ./builddir/todo
```

### Release Build and Installation

```
meson setup builddir --prefix=/usr
meson compile -C builddir
meson install -C builddir --destdir=/tmp/install
```

### Local Installation

For testing, install to a local prefix:

```
meson setup builddir --prefix=$HOME/.local
meson compile -C builddir
meson install -C builddir
~/.local/bin/todo
```

## Cargo.toml Changes

The Meson version no longer needs `glib-build-tools` since resources are compiled by Meson:

```
cargo remove --build glib-build-tools
```
