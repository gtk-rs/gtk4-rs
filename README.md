# gtk4-rs ![CI](https://github.com/gtk-rs/gtk4-rs/workflows/CI/badge.svg)

A group of crates that aims to provide complete [GTK](https://gtk.org/) 4 bindings. This repository contains all the "core" crates of GTK 4.

- [GTK](./gtk4)
- [GDK](./gdk4): An intermediate layer which isolates GTK from the details of the windowing system.
  - [GDK Macos](./gdk4-macos): Macos backend specific functions.
  - [GDK Wayland](./gdk4-wayland): Wayland backend specific functions.
  - [GDK Win32](./gdk4-win32): Windows backend specific functions.
  - [GDK X11](./gdk4-x11): X backend specific functions.
- [GSK](./gsk4): An intermediate layer which isolates GTK from the details of the OpenGL or Vulkan implementation.

The GTK 4 crates also depends on other libraries part of the platform like:

- [GLib](https://github.com/gtk-rs/gtk-rs-core/tree/main/glib)
- [Gio](https://github.com/gtk-rs/gtk-rs-core/tree/main/gio)
- [Graphene](https://github.com/gtk-rs/gtk-rs-core/tree/main/graphene)
- [Cairo](https://github.com/gtk-rs/gtk-rs-core/tree/main/cairo)
- [Pango](https://github.com/gtk-rs/gtk-rs-core/tree/main/pango)

Those are common with the GTK 3 and GStreamer bindings and are part of the [gtk-rs-core](https://github.com/gtk-rs/gtk-rs-core) repository.

For more information about each crate, please refer to their `README.md` file in their directory.

## Minimum supported Rust version

Currently, the minimum supported Rust version is `1.83`.

## Documentation

- [Examples](https://github.com/gtk-rs/gtk4-rs/tree/main/examples)
- Book [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- The Rust API [Stable](https://gtk-rs.org/gtk4-rs/stable/latest/docs/) / [Development](https://gtk-rs.org/gtk4-rs/git/docs/)

### Useful links

- [Website](https://gtk-rs.org)
- [Discourse](https://discourse.gnome.org/)
- Matrix: [#rust:gnome.org](https://matrix.to/#/#rust:gnome.org)

## Ecosystem

The `gtk4-rs` repository contains Rust crates for GTK 4. However there is a large ecosystem of `GObject` libraries and many of these
libraries have Rust bindings based on the tooling included in `gtk-rs`.
Of particular note:

* [gtk-rs-core](https://github.com/gtk-rs/gtk-rs-core) - bindings for some of the core libraries such as `glib`, `gio`, `pango`, `graphene`
* [gstreamer-rs](https://gitlab.freedesktop.org/gstreamer/gstreamer-rs) - bindings for the GStreamer media framework

Additionally, Rust bindings for various libraries are hosted on
[GNOME's GitLab](https://gitlab.gnome.org) instance and can be found at
<https://gitlab.gnome.org/World/Rust>.

When using crates that are not part of the `gtk4-rs` repository, you will
need to be careful and ensure that they do not pull in incompatible versions of core
crates like `glib-rs`.

### Built with GTK4 Rust bindings

A small list of applications built with gtk4-rs:

- [Solanum](https://gitlab.gnome.org/World/Solanum): A pomodoro timer
- [Shortwave](https://gitlab.gnome.org/World/Shortwave): An internet radio player
- [Authenticator](https://gitlab.gnome.org/World/Authenticator): A two-factor code generator
- [Health](https://gitlab.gnome.org/World/Health): A health tracking app
- [Video Trimmer](https://gitlab.gnome.org/YaLTeR/video-trimmer): A fast video trimmer

Libraries built with gtk4-rs:

- [Relm4](https://github.com/Relm4/relm4): An idiomatic GUI library inspired by Elm

## AI Contribution Policy

gtk-rs is a project by humans for humans. We prefer contributions that
are produced by human creativity, we expect a human to take full
responsibility for each contribution, and we will take more joy in
reviewing contributions when there's people at the other end of the
line to stand by their changes.

If you use LLM/GenAI tools for your contributions, here are the rules
you must follow:

### Requirements

1. Use AI as a tool. Verify behavior, correctness, and compatibility
   yourself prior to submitting your contribution. Do not ask the
   maintainers to do this for you.
1. Keep changes narrow and limited. Do **NOT** use LLM/GenAI tools to
   generate broad rewrites, large refactorings, or style changes.
1. Do **NOT** submit generated code, documentation, or tests that you
   don’t understand.
1. Do **NOT** fabricate benchmarks, bug reports, test results, code
   samples, or reproducers.
1. Do **NOT** include private code, credentials, tokens, or any other
   confidential material.
1. Respect the licensing and attribution requirements.

### Disclosure

Always disclose the use of LLM/GenAI tools when creating an issue or
a merge request. Do not include trailers like “Co-authored-by:” or
“Assisted-by:” in commit messages, since they serve as free advertising
for AI companies.

### Reviews

1. Describe your changes, and the verification steps.
1. Be prepared to explain all the changes yourself.
1. Do **NOT** feed the review feedback to an LLM/GenAI tool.

### Maintainers expectations

1. Review LLM/GenAI-assisted contributions more strictly than any other contribution.
1. Require reproducibility in fixes and tests.
1. Reject changes that appear to be unverified LLM/GenAI output.
1. Reject comments and feedback that appear to be LLM/GenAI output.

> A COMPUTER CAN NEVER BE HELD ACCOUNTABLE.
> THEREFORE A COMPUTER MUST NEVER MAKE A MAINTENANCE DECISION.

## Contributing

The bindings are composed of two parts:

- Automatically generated ones using [gir]
- Manual parts

The automatic ones can be generated using the [`generator.py`](https://github.com/gtk-rs/gir/blob/main/generator.py) script

```bash
python3 ./generator.py
```

If you didn't do so yet, please check out all the submodules before via

```bash
$ git submodule update --checkout
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
