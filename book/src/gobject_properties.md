# Properties

Signals are very useful, but when it comes to dealing with GObject state, properties are preferred over raw signals.
With properties you get:
- setters and getters
- emitted "notify" signal whenever it gets set
- ability to bind properties of different GObjects

Typically, a property corresponds to a single member of your GObject struct.
You will like also want to use `Cell` instead of `RefCell` for your property member.
Unlike `RefCell`, `Cell` has no overhead but only allows you to swap the value — not modify it.

Since properties already provide a "notify" signal, we can drop our "number-changed" signal and create a property "number" instead.


<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_1/src/main.rs:object_impl}}
```

The `properties` method describes our set of properties.
We only want a single one, so we give it a name ("number"), describe its type, range and default value. We also declare that the property can be read and be written to.
With `set_property` the underlying value can be changed. It also emits the corresponding "notify" signal.
With `get_property` the underlying value will be returned.

Now, we have to make sure to modify `number` via `set_property` to assure that the "notify" signal gets emitted.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_1/src/main.rs:button_impl}}
```

Connecting to the signal is very similar to the example in the Signal section.
We only exchange `connect_local` with `connect_notify_local` and specify the property as its first parameter. 

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_1/src/main.rs:label}}
```

This is already much nicer than emitting the signals ourselves, but that was mostly accounted to the fact that the number was only held by the single `CustomButton` instance.

Let us see how we could synchronize two `CustomButton` instances.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_2/src/main.rs:buttons}}
```

Properties can be bound to each other.
In our case, we want to bind the "number" property of `button_1` to the "number" property of `button_2`.
We also want the binding to be bidirectional, so we specify this with the [`BindingFlags`](http://gtk-rs.org/docs/glib/struct.BindingFlags.html).

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_2/src/main.rs:bind_number}}
```

This nearly enough, but if we now press on one button, the label of the other one gets not changed.
Luckily, "label" is a built-in property of `Button`, the class we inherited `CustomButton` from.
All we have to do is to bind the "label" property of `button_1` to the "label" property of `button_2`.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_properties_2/src/main.rs:bind_label}}
```

If we now click on one button, the "number" and the "label" property of the other button changes as well.

<div style="text-align:center"><img src="img/gobject_properties.png" /></div>