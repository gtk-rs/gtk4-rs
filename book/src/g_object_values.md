# Generic Values

Some GObject-related functions rely on generic values for their arguments or return parameters.
Since GObject introspection works through a C interface, these functions cannot rely on any powerful Rust concepts.
In these cases [`glib::Value`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/struct.Value.html) or [`glib::Variant`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/variant/struct.Variant.html) are used.

## Value

Let's start with `Value`.
Conceptually, a `Value` is similar to a Rust `enum` defined like this:

```rust, no_run,noplayground
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
    Object(Option<dyn IsA<glib::Object>>),
}
```

For example, this is how you would use a `Value` representing an `i32`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_values/1/main.rs">listings/g_object_values/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_values/1/main.rs:i32}}
```

Also note that in the `enum` above boxed types such as `String` or `glib::Object` are wrapped in an `Option`.
This comes from C, where every boxed type can potentially be `None` (or `NULL` in C terms).
You can still access it the same way as with the `i32` above.
[`get`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/struct.Value.html#method.get) will then not only return `Err` if you specified the wrong type, but also if the `Value` represents `None`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_values/1/main.rs">listings/g_object_values/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_values/1/main.rs:string}}
```

If you want to differentiate between specifying the wrong type and a `Value` representing `None`, just call `get::<Option<T>>` instead.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_values/1/main.rs">listings/g_object_values/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_values/1/main.rs:string_none}}
```

We will use `Value` when we deal with properties and signals later on.

## Variant

A `Variant` is used whenever data needs to be serialized, for example for sending it to another process or over the network, or for storing it on disk.
Although `GVariant` supports arbitrarily complex types, the Rust bindings are currently limited to `bool`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `f64`, `&str`/`String`, and [`VariantDict`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.VariantDict.html).
Containers of the above types are possible as well, such as `HashMap`, `Vec`, `Option`, tuples up to 16 elements, and `Variant`.
Variants can even be [derived](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib_macros/derive.Variant.html#) from Rust structs as long as its members can be represented by variants.

In the most simple case, converting Rust types to `Variant` and vice-versa is very similar to the way it worked with `Value`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_values/2/main.rs">listings/g_object_values/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_values/2/main.rs:i32}}
```

However, a `Variant` is also able to represent containers such as [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) or [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
The following snippet shows how to convert between `Vec` and `Variant`.
More examples can be found in the [docs](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/variant/index.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_values/2/main.rs">listings/g_object_values/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_values/2/main.rs:vec}}
```

We will use `Variant` when saving settings using [`gio::Settings`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Settings.html) or activating actions via [`gio::Action`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Action.html).
