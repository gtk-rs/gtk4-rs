# Properties

Properties provide a public API for accessing state of GObjects.

Let us see how this is done by experimenting with the `Switch` widget.
One of its properties is the [state](https://docs.gtk.org/gtk4/property.Switch.state.html).
According to the GTK docs, it can be read and be written to.
That is why, `gtk-rs` provides corresponding [`state`](../docs/gtk4/struct.Switch.html#method.state) and [`set_state`](../docs/gtk4/docs/gtk4/struct.Switch.html#method.set_state) methods.

<span class="filename">Filename: listings/gobject_properties/1/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/1/main.rs:switch}}
```
Alternatively, we can use the general [`property`](http://gtk-rs.org/docs/glib/object/trait.ObjectExt.html#tymethod.property) and [`set_property`](http://gtk-rs.org/docs/glib/object/trait.ObjectExt.html#tymethod.set_property) methods.
Because they can be used for properties of different types, they operate with `glib::Value`.

<span class="filename">Filename: listings/gobject_properties/2/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/2/main.rs:switch}}
```

Dealing with a `glib::Value` is quite verbose
This is why you will only want to use the generic methods for accessing properties you have added to your custom GObjects.

Properties can not only be accessed via getters & setters, they can also be bound to each other.
Let us see how that would look like for two `Switch` instances.

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:switches}}
```

In our case, we want to bind the "state" property of `switch_1` to the "state" property of `switch_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](http://gtk-rs.org/docs/glib/struct.BindingFlags.html).

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:bind_state}}
```

Whenever we now click on one of the two switches, the other one gets toggled as well.

<div style="text-align:center"><img src="img/gobject_properties_switches.png" /></div>

We can also add properties to custom GObjects.
We can demonstrate that, by binding the `number` of our `CustomButton` to a property.
For that, we need to be able to [lazily evaluate](https://en.wikipedia.org/wiki/Lazy_evaluation) expressions.
The crate `once_cell` provides the `Lazy` type which allows us to do that.
`once_cell` is already part of Rust nightly.
Until it hits stable, we will add it as external dependency.

<span class="filename">Filename: listings/Cargo.toml</span>

```toml
[dependencies]
once_cell = "1"
```

Now we define the "number" property within the `ObjectImpl` implementation.
The `properties` method describes our set of properties.
When naming our property, we make sure to do that in [kebab-case](https://wiki.c2.com/?KebabCase).
Then, we describe its type, range and default value.
We also declare that the property can be read and be written to.
`set_property` describes how the underlying values can be changed.
`property` takes care of returning the underlying value.
The formerly private `number` is now accessible via the `property` and `set_property` methods.

<span class="filename">Filename: listings/gobject_properties/4/custom_button/imp.rs</span>

```rust
{{#rustdoc_include ../listings/gobject_properties/4/custom_button/imp.rs:object_impl}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

We can immediately take advantage of this new property by binding the "label" property to it.
It even converts the integer value of "number" to the string of "label".
Now we do not have to adapt the label in the "clicked" callback anymore.

Let us see what we can do with this by creating two custom buttons.

<span class="filename">Filename: listings/gobject_properties/4/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/4/main.rs:buttons}}
```

We have already seen that bound properties do not necessarily have to be of the same type.
By leveraging [`transform_to`](http://gtk-rs.org/docs/glib/object/struct.BindingBuilder.html#method.transform_to) and [`transform_from`](http://gtk-rs.org/docs/glib/object/struct.BindingBuilder.html#method.transform_from), we can assure that `button_2` always displays a number which is 1 higher than the number of `button_1`.

<span class="filename">Filename: listings/gobject_properties/4/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/4/main.rs:bind_numbers}}
```
If we now click on one button, the "number" and "label" properties of the other button change as well.

<div style="text-align:center"><img src="img/gobject_properties_buttons.png"/></div>

The final nice feature of properties is, that you can connect a callback to the event when a property gets changed.
We can do this like this:

<span class="filename">Filename: listings/gobject_properties/4/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/4/main.rs:connect_notify}}
```

Now, whenever the "number" property gets changed, the closure gets executed and prints the current value of "number".

You will want to introduce properties to your custom GObjects whenever you want
- to allow consumers to be able to access internal state,
- to bind state of (different) GObjects or
- to notify consumers whenever the property value changes.


