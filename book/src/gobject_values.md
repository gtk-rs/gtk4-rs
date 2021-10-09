# Generic Values

Some GObject-related functions rely on generic values for their arguments or return parameters.
Since GObject introspection works through a C interface, these functions cannot rely on any powerful Rust concepts.
In these cases [`glib::Value`](http://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/value/struct.Value.html) or [`glib::Variant`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/variant/struct.Variant.html) are used.

## Value

Let us start with `Value`.
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

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_values/1/main.rs:i32}}
```

Also note that in the `enum` above boxed types such as `String` or `glib::Object` are wrapped in an `Option`.
This comes from C, where every boxed type can potentially be `None` (or `NULL` in C terms).
You can still access it the same way as with the `i32` above.
`get` will then not only return `Err` if you specified the wrong type, but also if the `Value` represents `None`.

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_values/1/main.rs:string}}
```

If you want to differentiate between specifying the wrong type and a `Value` representing `None`, just call `get::<Option<T>>` instead.

<span class="filename">Filename: listings/gobject_values/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_values/1/main.rs:string_none}}
```

We will use `Value` when we deal with properties and signals later on.

## Variant

A `Variant` is used whenever data needs to be serialized, for example for sending it to another process or over the network, or for storing it on disk.
In that sense, a `Variant` is similar to a Rust object that implements [`serde::Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`serde::Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html).
It would be possible to add derive macros similar to the ones of `serde` for serializing/deserializing arbitrary Rust structs.
It is just that nobody did the work yet.
Pull requests to add this functionality would be very appreciated!

Although `GVariant` supports arbitrarily complex types, the Rust bindings are currently limited to `bool`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `f64`, `&str`/`String`, and [`VariantDict`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.VariantDict.html).
Containers of the above types are possible as well, such as `HashMap`, `Vec`, `Option`, tuples up to 16 elements, and `Variant`.
These can even be arbitrarily nested like e.g. `HashMap<Option<u32>, Vec<HashMap<String, Variant>>>`.

In the most simple case, converting Rust types to `Variant` and vice-versa is very similar to the way it worked with `Value`.

<span class="filename">Filename: listings/gobject_values/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_values/2/main.rs:i32}}
```

However, a `Variant` is also able to represent containers such as [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) or [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
The following snippet shows how to convert between `Vec` and `Variant`.
More examples can be found in the [docs](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/variant/index.html).

<span class="filename">Filename: listings/gobject_values/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_values/2/main.rs:vec}}
```

We will use `Variant` when saving settings using `gio::Settings` or activating actions via `gio::Action`.
