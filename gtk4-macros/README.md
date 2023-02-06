# gtk4-macros

[Project site](https://gtk-rs.org/)

Macro helpers for GTK 4 bindings, part of [gtk4-rs](https://github.com/gtk-rs/gtk4-rs/).

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.64.0`.

## Documentation

- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4_macros)/[Development](https://gtk-rs.org/gtk4-rs/git/docs/gtk4_macros/)

## Available Macros

- `CompositeTemplate`

### Features

| Feature | Description |
| ---     | ----------- |
| `xml_validation` | Check the existence of `#[template_child]` fields in the UI file. Only works with `#[template(string = "")]` |
| `blueprint` | Adds blueprint usage support in `#[template(string = "")]` |

### See Also

- [glib](https://crates.io/crates/glib)
- [gio](https://crates.io/crates/gio)
- [gtk4](https://crates.io/crates/gdk4)
- [gdk4](https://crates.io/crates/gdk4)
- [gtk4](https://crates.io/crates/gtk4)

## License

The Rust bindings of __gtk4-macros__ are available under the MIT License, please refer to it.
