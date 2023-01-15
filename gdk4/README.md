# Rust GDK 4 bindings

The project website is [here](https://gtk-rs.org/).

Rust bindings of [GDK 4](https://docs.gtk.org/gdk4/), part
of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.64.0`.

## Documentation

- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gdk4)/[Development](https://gtk-rs.org/gtk4-rs/git/docs/gdk4/)
- [The C API](https://docs.gtk.org/gdk4/)
- [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#library-versions).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gdk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gtk = {version = "0.1", package = "gtk4"}
gdk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gdk4" }
```

### Features

| Feature | Description |
| ---     | ----------- |
| `v4_10` | Enable the new APIs part of the to be released GTK 4.10 |
| `v4_8` | Enable the new APIs part of GTK 4.8 |
| `v4_6` | Enable the new APIs part of GTK 4.6 |
| `v4_4` | Enable the new APIs part of GTK 4.4 |
| `v4_2` | Enable the new APIs part of GTK 4.2 |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gsk4](https://crates.io/crates/gsk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gdk4__ are available under the MIT License, please refer to it.
