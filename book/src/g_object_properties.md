# Properties

Properties provide a public API for accessing state of GObjects.

Let's see how this is done by experimenting with the [`Switch`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Switch.html) widget.
One of its properties is the [state](https://docs.gtk.org/gtk4/property.Switch.state.html).
According to the GTK docs, it can be read and be written to.
That is why `gtk-rs` provides corresponding [`state`](../docs/gtk4/struct.Switch.html#method.state) and [`set_state`](../docs/gtk4/struct.Switch.html#method.set_state) methods.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/1/main.rs">listings/g_object_properties/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/1/main.rs:switch}}
```
Alternatively, we can use the general [`property`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/trait.ObjectExt.html#tymethod.property) and [`set_property`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/trait.ObjectExt.html#tymethod.set_property) methods.
We use the [turbofish](https://matematikaadit.github.io/posts/rust-turbofish.html) syntax to specify the type if it cannot be inferred.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/2/main.rs">listings/g_object_properties/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/2/main.rs:switch}}
```

Both `property` and `set_property` panic if the property does not exist, has the wrong type or has the wrong permissions.
This is fine in most situations where these cases are hard-coded within the program.
However, if you want to decide for yourself how to react to failure, you can enforce a returned `Option` by specifying `property::<Option<T>>` or `set_property::<Option<T>>`.

Properties can not only be accessed via getters & setters, they can also be bound to each other.
Let's see how that would look like for two `Switch` instances.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/main.rs">listings/g_object_properties/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/main.rs:switches}}
```

In our case, we want to bind the "state" property of `switch_1` to the "state" property of `switch_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.BindingFlags.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/main.rs">listings/g_object_properties/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/main.rs:bind_state}}
```

Now when we click on one of the two switches, the other one is toggled as well.

<div style="text-align:center">
 <video autoplay muted loop>
    <source src="vid/g_object_properties_switches.webm">
    Your browser does not support the video tag.
 </video>
</div>

## Adding Properties to Custom GObjects

We can also add properties to custom GObjects.
We can demonstrate that by binding the `number` of our `CustomButton` to a property.
For that we need to be able to [lazily evaluate](https://en.wikipedia.org/wiki/Lazy_evaluation) expressions.
The crate `once_cell` provides the `Lazy` type which allows us to do that.
`once_cell` is already part of Rust nightly.
Until it hits stable, we will add it as external dependency.

```
cargo add once_cell
```

Now we define the "number" property within the `ObjectImpl` implementation.
The `properties` method describes our set of properties.
When naming our property, we make sure to do that in [kebab-case](https://en.wikipedia.org/wiki/Letter_case#Kebab_case).
`set_property` describes how the underlying values can be changed.
`property` takes care of returning the underlying value.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/custom_button/imp.rs">listings/g_object_properties/4/custom_button/imp.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/4/custom_button/imp.rs:object_impl}}
```

We could immediately take advantage of this new property by binding the "label" property to it.
It even converts the integer value of "number" to the string of "label".
Now we don't have to adapt the label in the "clicked" callback anymore.

We also have to adapt the `clicked` method.
Before we modified `number` directly, now we do it through `set_property`.
This way the "notify" signal will be emitted which bindings work as expected.

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/4/custom_button/imp.rs:button_impl}}
```

Let's see what we can do with this by creating two custom buttons.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/main.rs">listings/g_object_properties/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/4/main.rs:buttons}}
```

We have already seen that bound properties don't necessarily have to be of the same type.
By leveraging [`transform_to`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.BindingBuilder.html#method.transform_to) and [`transform_from`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.BindingBuilder.html#method.transform_from), we can assure that `button_2` always displays a number which is 1 higher than the number of `button_1`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/main.rs">listings/g_object_properties/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/4/main.rs:bind_numbers}}
```
Now if we click on one button, the "number" and "label" properties of the other button change as well.

<div style="text-align:center">
 <video autoplay muted loop>
    <source src="vid/g_object_properties_buttons.webm">
    Your browser does not support the video tag.
 </video>
</div>

The final nice feature of properties is, that you can connect a callback to the event when a property gets changed.
For example like this:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/4/main.rs">listings/g_object_properties/4/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/4/main.rs:connect_notify}}
```

Now, whenever the "number" property gets changed, the closure gets executed and prints the current value of "number" to standard output.

Introducing properties to your custom GObjects is useful if you want to
- bind state of (different) GObjects
- notify consumers whenever a property value changes

Note that it has a (computational) cost to send a signal each time the value changes.
If you only want to expose internal state, adding getter and setter methods is the better option.
