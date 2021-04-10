# gtk4-rs ![CI](https://github.com/gtk-rs/gtk4-rs/workflows/CI/badge.svg)

A group of crates that aims to provide complete [GTK](https://gtk.org/) 4 bindings. This repository contains all the "core" crates of GTK 4.

- [GTK](./gtk4)
- [GDK](./gdk4): An intermediate layer which isolates GTK from the details of the windowing system.
  - [GDK Wayland](./gdk4-wayland): Wayland backend specific functions.
  - [GDK X11](./gdk4-x11): X backend specific functions.
- [GSK](./gsk4): An intermediate layer which isolates GTK from the details of the OpenGL or Vulkan implementation.

The GTK 4 crates also depends on other libraries part of the platform like:

- [GLib](https://github.com/gtk-rs/gtk-rs/tree/master/glib)
- [Gio](https://github.com/gtk-rs/gtk-rs/tree/master/gio)
- [Graphene](https://github.com/gtk-rs/gtk-rs/tree/master/graphene)
- [Cairo](https://github.com/gtk-rs/gtk-rs/tree/master/cairo)
- [Pango](https://github.com/gtk-rs/gtk-rs/tree/master/pango)

Those are common with the GTK 3 and GStreamer bindings and are part of the [gtk-rs](https://github.com/gtk-rs/gtk-rs) repository.

For more information about each crate, please refer to their `README.md` file in their directory.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.48.0`.

## Documentation

### Development Branch

- [Book](https://gtk-rs.org/gtk4-rs/git/book)
- [GTK](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/)
- [GDK](https://gtk-rs.org/gtk4-rs/git/docs/gdk4/)
- [GSK](https://gtk-rs.org/gtk4-rs/git/docs/gsk4/)
- [GDK Wayland](https://gtk-rs.org/gtk4-rs/git/docs/gdk4_wayland/)
- [GDK X11](https://gtk-rs.org/gtk4-rs/git/docs/gdk4_x11/)
- [GTK Macros](https://gtk-rs.org/gtk4-rs/git/docs/gtk4_macros/)

## Contributing

The bindings are composed of two parts:

- Automatically generated ones using [gir]
- Manual parts

The automatic ones can be generated using the `generator.py` script

```bash
python3 generator.py
```

All the crates except `gtk4-macros` follow this structure

```text
   ./crate 
   ├── Gir.toml
   ├── README.md
   ├── src
   │   ╰── auto
   ├── sys
   ╰── tests
```

- `README.md`: Explanations about the crate itself and eventually some details.
- `Gir.toml`: Used by [gir] to generate most of the code.
- `src`: Contains the source code of the crate.
- `src/auto`: Contains the automatically generated part of the source code.
- `sys`: Contains the 1:1 bindings of the C API.

[gir]: https://github.com/gtk-rs/gir
