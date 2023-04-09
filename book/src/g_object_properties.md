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

Properties can not only be accessed via getters & setters, they can also be bound to each other.
Let's see how that would look like for two `Switch` instances.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/2/main.rs">listings/g_object_properties/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/2/main.rs:switches}}
```

In our case, we want to bind the "state" property of `switch_1` to the "state" property of `switch_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.BindingFlags.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/2/main.rs">listings/g_object_properties/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/2/main.rs:bind_state}}
```

Now when we click on one of the two switches, the other one is toggled as well.

<div style="text-align:center">
 <video autoplay muted loop>
    <source src="vid/g_object_properties_switches.webm">
    <p>A video which shows that toggling one button also toggles the other one </p>
 </video>
</div>

## Adding Properties to Custom GObjects

We can also add properties to custom GObjects.
We can demonstrate that by binding the `number` of our `CustomButton` to a property.
The `properties` method describes our set of properties.
When naming our property, we make sure to do that in [kebab-case](https://en.wikipedia.org/wiki/Letter_case#Kebab_case).
`set_property` describes how the underlying values can be changed.
`property` takes care of returning the underlying value.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/custom_button/imp.rs">listings/g_object_properties/3/custom_button/imp.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/custom_button/imp.rs:object_impl}}
```

We could immediately take advantage of this new property by binding the "label" property to it.
It even converts the integer value of "number" to the string of "label".
Now we don't have to adapt the label in the "clicked" callback anymore.

We also have to adapt the `clicked` method.
Before we modified `number` directly, now we do it through `set_property`.
This way the "notify" signal will be emitted which bindings work as expected.

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/custom_button/imp.rs:button_impl}}
```

Let's see what we can do with this by creating two custom buttons.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/main.rs">listings/g_object_properties/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/main.rs:buttons}}
```

We have already seen that bound properties don't necessarily have to be of the same type.
By leveraging [`transform_to`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.BindingBuilder.html#method.transform_to) and [`transform_from`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/struct.BindingBuilder.html#method.transform_from), we can assure that `button_2` always displays a number which is 1 higher than the number of `button_1`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/main.rs">listings/g_object_properties/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/main.rs:bind_numbers}}
```
Now if we click on one button, the "number" and "label" properties of the other button change as well.

<div style="text-align:center">
 <video autoplay muted loop>
    <source src="vid/g_object_properties_buttons.webm">
    <p>A video which shows that pressing on one button also changes the number on the other one</p>
 </video>
</div>

The final nice feature of properties is, that you can connect a callback to the event when a property gets changed.
For example like this:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_properties/3/main.rs">listings/g_object_properties/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_properties/3/main.rs:connect_notify}}
```

Now, whenever the "number" property gets changed, the closure gets executed and prints the current value of "number" to standard output.

Introducing properties to your custom GObjects is useful if you want to
- bind state of (different) GObjects
- notify consumers whenever a property value changes

Note that it has a (computational) cost to send a signal each time the value changes.
If you only want to expose internal state, adding getter and setter methods is the better option.
