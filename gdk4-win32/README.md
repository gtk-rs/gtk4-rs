# Rust GDK 4 Win32 bindings

The project website is [here](https://gtk-rs.org/).

Rust bindings of [GDK 4's Win32 backend](https://docs.gtk.org/gdk4-win32/),
part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

GDK is an intermediate layer that isolates GTK from the details of the windowing system.
GDK Win32 contains functions specific to the Win32 backend.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.70`.

## Documentation

- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gdk4_win32)/[Development](https://gtk-rs.org/gtk4-rs/git/docs/gdk4_win32/)
- [The C API](https://docs.gtk.org/gdk4-win32/)
- [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#library-versions).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gdk-win32= { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4-win32" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk-x11 = {version = "0.1", package = "gdk4-x11"}
gdk-win32 = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4-win32" }
```

### Features

| Feature | Description |
| ---     | ----------- |
| `v4_4` | Enable the new APIs part of GTK 4.4 |
| `egl` | Integration with the [khronos-egl](https://crates.io/crates/khronos-egl) crate |
| `win32` | Integration with the [windows](https://crates.io/crates/windows) crate |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gsk4](https://crates.io/crates/gsk4)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gdk4-win32__ are available under the MIT License, please refer to it.
