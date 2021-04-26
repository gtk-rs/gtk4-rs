# Generic Values

Some GObject-related functions rely on generic values for their arguments or return parameters.
Enums in C are not as powerful as the one in Rust, which is why [`glib::Value`](http://gtk-rs.org/docs/glib/value/struct.Value.html) is used for this.
Conceptually though, there are similar to a Rust enum defined like this:

```rust
enum Value {
    bool,
    i8,
    i32,
    u32,
    i64,
    u64,
    f32,
    f64,
    String,
    Object,
}
```

That means that a `glib::Value` can represent a set of types, but only one of them per instance.
For example, this is how you would use a `Value` representing an `i32`.

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_values/1/main.rs}}
```

For now, all you need to know is how to convert a Rust type into a `glib::Value` and vice versa.
This knowledge will be useful in the next chapters where we discuss properties, signals and models.
