# Project Setup

Let's begin by installing all necessary tools. First, follow the instructions
on the [GTK website](https://www.gtk.org/docs/installations/) in order to
install GTK 4. Then install Rust with [rustup](https://rustup.rs/).

Now, create a new project by executing:

```bash
cargo new my-gtk-app
```

Add the following line to your dependencies in `Cargo.toml`, where `X.Y.Z`
should be replaced with the most up-to-date version of the
[gtk4 crate](https://crates.io/crates/gtk4).

```toml
gtk = { version = "X.Y.Z", package = "gtk4" }
```

You can also obtain the value of `X.Y.Z` by searching through `cargo search`
using the following command:

```bash
cargo search gtk4 --limit 1
```

> To use functionality that has been added to later releases, you have to specify
> this as a [feature](https://doc.rust-lang.org/cargo/reference/features.html).
> You can obtain a list of supported features at [gtk4
> crate](https://crates.io/crates/gtk4).
>
> By default `gtk4-rs` is compatible with all GTK 4 releases.  For example, if
> you want to use functionality that was introduced with GTK 4.6, you would add
> the following to your `gtk` dependency in `Cargo.toml`.
>
> ```toml
> gtk = { version = "X.Y.Z", package = "gtk4", features = ["v4_6"]}
> ```
>
> This will only work if your available GTK version is indeed >= 4.6.
> You can get the version by executing the following command:
>
> ```bash
> pkg-config --modversion gtk4
> ```

Now, you can run your application by executing:

```bash
cargo run
```
