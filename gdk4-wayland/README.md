# gdk4-wayland

[Project site](https://gtk-rs.org/)

Rust bindings of __GDK 4's Wayland backend__, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

- [Rust API - Stable] TODO
- [Rust API - Development](https://gtk-rs.org/gtk4-rs/git/docs/gdk4_wayland/)
- [C API](https://docs.gtk.org/gdk4-wayland/)
- [GTK Installation instructions](https://www.gtk.org/docs/installations/)


## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gdk-wayland = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4-wayland" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk-x11 = {version = "0.1", package = "gdk4-x11"}
gdk-wayland = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4-wayland" }
```

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gsk4](https://crates.io/crates/gsk4)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gdk4-wayland__ are available under the MIT License, please refer to it.
