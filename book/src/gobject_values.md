# Generic Values

Some GObject-related functions rely on generic values for their arguments or return parameters.
Enums in C are not as powerful as the one in Rust, which is why [`glib::Value`](http://gtk-rs.org/docs/glib/value/struct.Value.html) is used for this.
Conceptually though, there are similar to a Rust enum defined like this:

```rust
enum Value <T> {
    bool(bool),
    i8(i8),
    i32(i32),
    u32(u32),
    i64(i64),
    u64(u64),
    f32(f32),
    f64(f64),
    // boxed types
    String(Option<String>),
    Object(Option<T: impl IsA<Object>>),
```

That means that a `glib::Value` can represent a set of types, but only one of them per instance.
For example, this is how you would use a `Value` representing an `i32`.

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_values/1/main.rs:i32}}
```

Please note that boxed types such as `String` or `Object` are wrapped in an `Option`.
This comes from C, where boxed types can always be `None` (or null in C terms).
You can still access it the same way as with the `i32` above.
`get` will then not only return `Err` if you specified the wrong type, but also if the `Value` represents `None`.

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_values/1/main.rs:string}}
```

If you want to differentiate between specifying the wrong type and a `Value` representing `None`, just call `get::<Option<T>>` instead.

```rust,no_run
{{#rustdoc_include ../listings/gobject_values/1/main.rs:string_none}}
```

For now, all you need to know is how to convert a Rust type into a `glib::Value` and vice versa.
This knowledge will be useful in the next chapters where we discuss properties, signals and models.
