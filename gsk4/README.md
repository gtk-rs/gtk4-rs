# gsk4

[Project site](https://gtk-rs.org/)

Rust bindings of __GSK 4__, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

__Required Rust version__: 1.48+.

## Documentation

- [Stable Version] TODO
- [Development Version](https://gtk-rs.org/gtk4-rs/docs/git/gsk4/)
- [The C API](https://docs.gtk.org/gsk4/)
- [GTK Installation instructions](https://www.gtk.org/docs/installations/)

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](https://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
gsk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gsk4" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gdk = {version = "0.1", package = "gdk4"}
gsk = { git = "https://github.com/gtk-rs/gtk4-rs.git", package = "gsk4" }
```

### Features

| Feature | Description |
| ---     | ----------- |
| `vulkan` | Enable `vulkan` Renderer. Only if GTK is compiled with Vulkan support |
| `broadway` | Enable `broadway` Renderer. Only if GTK is compiled with Broadway support |
| `v4_2` | Enable the new APIs part of GTK 4.2 |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gsk4__ are available under the MIT License, please refer to it.
