# Properties

Properties allow us to access state of GObjects.
Let us take a closer look at `Switch` again.
One of its properties is the [state](https://docs.gtk.org/gtk4/property.Switch.state.html).
It can be read as well as be written to.
We do that by calling the `get_property` and `set_property` methods.
Since all of this is highly dynamic, checks occur at runtime and the code involves more boilerplate.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_1/src/main.rs:switch}}
```

Properties can also be bound to each other.
We could, for example, with two `Switch` instances.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_2/src/main.rs:switches}}
```

In our case, we want to bind the "state" property of `switch_1` to the "number" property of `switch_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](http://gtk-rs.org/docs/glib/struct.BindingFlags.html).

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_2/src/main.rs:bind_state}}
```

Whenever we now click on one of the two switches, the other one gets toggled as well.

<div style="text-align:center"><img src="img/gobject_properties.png" /></div>


We can also add properties to custom GObjects.
The `CustomButton` from last section holds a `number`, which is a good candidate for mapping it to a property.
First we have to define the property within the `ObjectImpl` implementation.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_3/src/main.rs:object_impl}}
```

The `properties` method describes our set of properties.
We only want a single one, so we give it a name, describe its type, range and default value. We also declare that the property can be read and be written to.
`set_property` describes how the underlying values can be changed.
`get_property` takes care of returning the underlying value.

The formerly private `number` is now accessible via the `get_property` and `set_property` methods.
Let us see how we could synchronize two `CustomButton` instances, both with their own `number`.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_3/src/main.rs:buttons}}
```

However, but if we now press on one button, the label of the other one does not get updated.
Luckily, "label" is a built-in property of `Button`, the class from which `CustomButton` inherits of.
All we have to do is to bind the "label" property of `button_1` to the "label" property of `button_2`.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_3/src/main.rs:bind_label}}
```

If we now click on one button, the "number" and "label" properties of the other button change as well.

<div style="text-align:center"><img src="img/gobject_properties.png"/></div>

You will want to introduce properties to your custom GObjects whenever you want:
- consumers to access internal state
- to bind state of GObjects
- an emitted "notify" signal whenever the property gets set


