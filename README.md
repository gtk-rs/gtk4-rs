# gtk4-rs

The `gtk4-rs` are a group of crates that aims to provide gtk4 bindings.

This repository contains all the "core" crates of gtk 4. For more
information about each crate, please refer to their `README.md` file in their directory.

## Documentation

### Development Branch

 * [gtk](https://gtk-rs.org/gtk4-rs/gtk4/)
 * [gdk](https://gtk-rs.org/gtk4-rs/gdk4/)
 * [gsk](https://gtk-rs.org/gtk4-rs/gsk4/)

## Regenerating

To regenerate crates using [gir], please use the `generator.py`
file as follows:

```bash
$ python3 generator.py
```

## Development


This repository is structured as follows:

```text
- crate/
   |
   |-- README.md
   |-- Gir.toml
   |-- Cargo.toml
   |-- src/
   |-- sys/
```

The `crate` is a "top" directory (so "gdk4" or "gtk4" in here for example). I listed some
import files, let's quickly explain them:

 * `README.md`: Explanations about the crate itself and eventually some details.
 * `Gir.toml`: Used by [gir] to generate most of the crates' code.
 * `Cargo.toml`: File describing the crate, used by `cargo` and `Rust`.
 * `src`: Contains the source code of the crate.
 * `sys`: Contains the 1:1 bindings of the C API.

[gir]: https://github.com/gtk-rs/gir
