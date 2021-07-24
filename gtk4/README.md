# gtk4

[Project site](https://gtk-rs.org/)

Rust bindings of __GTK 4__, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.53.0`.

## Documentation

- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4)/[Development](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/)
- Book [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/book)/[Development](https://gtk-rs.org/gtk4-rs/git/book)
- [Examples](https://github.com/gtk-rs/gtk4-rs/tree/master/examples)
- [The C API](https://docs.gtk.org/gtk4/)
- [GTK Installation Instructions](https://www.gtk.org/docs/installations/)


## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html#library-versions).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk = {version = "0.1", package = "gdk4"}
gtk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gtk4" }
```

### Features

| Feature | Description |
| ---     | ----------- |
| `v4_2` | Enable the new APIs part of GTK 4.2 |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gdk4](https://crates.io/crates/gdk4)
- [gsk4](https://crates.io/crates/gsk4)

## License

The Rust bindings of __gtk4__ are available under the MIT License, please refer to it.
