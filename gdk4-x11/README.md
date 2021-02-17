# gdk4-x11

[Project site](https://gtk-rs.org/)

__Rust__ bindings and wrappers for __GDK 4's X11 backend__.

## Documentation
- [Stable] TODO
- [Developement](https://gtk-rs.org/gtk4-rs/gdk4_x11/)
- [The C documentation](https://developer.gnome.org/gdk4/stable/gdk4-X-Window-System-Interaction.html)


## Building

__gdk4-x11__ expects the developement files of various libraries to be installed on your machine.You can follow [the installation instructions](https://www.gtk.org/docs/installations/)

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

### See Also:

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gsk4](https://crates.io/crates/gsk4)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4](https://crates.io/crates/gtk4)


## License

The Rust bindings of __gdk4-x11__ are available under the MIT License, please refer to it.
