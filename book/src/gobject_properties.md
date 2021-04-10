# Properties

Properties allow us to access state of GObjects.

Let us see how this is done by experimenting with the `Switch` widget.
One of its properties is the [state](https://docs.gtk.org/gtk4/property.Switch.state.html).
It can be read and be written to.
We do that by calling the `get_property` and `set_property` methods.
Since all of this is highly dynamic, checks occur at runtime and the code involves a bit of boilerplate.

<span class="filename">Filename: listings/gobject_properties/1/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/1/main.rs:switch}}
```

Properties can also be bound to each other.
Let us see how would bind the properties of two `Switch` instances.

<span class="filename">Filename: listings/gobject_properties/2/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/2/main.rs:switches}}
```

In our case, we want to bind the "state" property of `switch_1` to the "state" property of `switch_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](http://gtk-rs.org/docs/glib/struct.BindingFlags.html).

<span class="filename">Filename: listings/gobject_properties/2/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/2/main.rs:bind_state}}
```

Whenever we now click on one of the two switches, the other one gets toggled as well.

<div style="text-align:center"><img src="img/gobject_properties_switches.png" /></div>


We can also add properties to custom GObjects.
We can demonstrate that, by binding the `number` of our `CustomButton` to a property.

However, we first need to add `once_cell` to our dependencies.
With this, we can [lazily evaluate](https://en.wikipedia.org/wiki/Lazy_evaluation) expressions, which we often need when working with custom GObjects.

```toml
[dependencies]
once_cell = "1"
```

Then we have to define the property within the `ObjectImpl` implementation.

<span class="filename">Filename: listings/gobject_properties/3/custom_button/imp.rs</span>

```rust
{{#rustdoc_include ../listings/gobject_properties/3/custom_button/imp.rs:object_impl}}
```

The `properties` method describes our set of properties.
When naming our property, we make sure to do that in [kebab-case](https://wiki.c2.com/?KebabCase).
Then, we describe its type, range and default value.
We also declare that the property can be read and be written to.
`set_property` describes how the underlying values can be changed.
`get_property` takes care of returning the underlying value.
The formerly private `number` is now accessible via the `get_property` and `set_property` methods.

Let us see what we can do with this by creating two custom buttons.

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:buttons}}
```

The numbers of the two buttons can now be bound to each other.

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:bind_number}}
```

The numbers are bound to each other, but if we now press on one button, the label of the other one does not get updated.
Luckily, "label" is a built-in property of `Button`, the class from which `CustomButton` inherits of.
All we have to do is to bind the "label" property of `button_1` to the "label" property of `button_2`.

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:bind_label}}
```

If we now click on one button, the "number" and "label" properties of the other button change as well.

<div style="text-align:center"><img src="img/gobject_properties_buttons.png"/></div>

The final nice feature of properties is, that you can connect a callback to the event when a property gets changed.
We can do this like this:

<span class="filename">Filename: listings/gobject_properties/3/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties/3/main.rs:connect_notify}}
```

Now, whenever the "number" property gets changed, the closure gets executed and prints the current value of "number".

You will want to introduce properties to your custom GObjects whenever you want
- to allow consumers to be able to access internal state,
- to bind state of (different) GObjects or
- to notify consumers whenever the property gets set.


