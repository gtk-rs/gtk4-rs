# Let To-Do App Follow GNOME's HIG

Within this chapter we will adapt our To-Do app so that it follow GNOME's [HIG](https://developer.gnome.org/hig/).
Let's start by installing libadwaita and adding the libadwaita crate to our dependencies as explained in the former [chapter](libadwaita.html).

The most simple change we can make is replacing [`gtk::Application`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Application.html) with [`adw::Application`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Application.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/main.rs">listings/todo/5/main.rs</a>

```rust,no_run,noplayground
{{#rustdoc_include ../listings/todo/5/main.rs:main}}
```

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/window/mod.rs">listings/todo/5/window/mod.rs</a>

```rust,no_run,noplayground
{{#rustdoc_include ../listings/todo/5/window/mod.rs:new}}
```

`adw::Application` calls [`init`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/functions/fn.init.html) internally and makes sure that translations, types, stylesheets, and icons are set up properly for Libadwaita. 
It also loads stylesheets automatically from resources as long as they are named [accordingly](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Application.html#automatic-resources).

Looking at our To-Do app we can see that the looks of its widgets changed.
This is because the Default stylesheet provided by GTK has been replaced with the Adwaita stylesheet provided by Libadwaita.

# TODO: Add comparison

If the setting is set to dark, then dark styles are loaded.

# TODO: Show transition

## Start using Libadwaita widgets
