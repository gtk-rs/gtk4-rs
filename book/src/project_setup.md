# Project Setup

Let's begin by installing all necessary tools.
First, follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) in order to install GTK 4.
Then install Rust with [rustup](https://rustup.rs/).

Now, create a new project by executing:
```bash
cargo new my-gtk-app
```

Add the [gtk4 crate](https://crates.io/crates/gtk4) to your dependencies in `Cargo.toml` using following command:

```
cargo add gtk4 --rename gtk
```

>To use functionality that has been added to later releases, you have to specify this as a [feature](https://doc.rust-lang.org/cargo/reference/features.html).
>Per default `gtk4-rs` is compatible with all GTK 4 releases.
>For example, if you want to use functionality that was introduced with GTK 4.6, you would run the following command:
>
> ```
> cargo add gtk4 --rename gtk --features v4_6
> ```
>
>This will only work if your available GTK version is indeed >= 4.6.
>You can get the version by executing the following command:
>```
>pkg-config --modversion gtk4
>```

Now, you can run your application by executing:
```bash
cargo run
```
