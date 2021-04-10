# Signals

GObject signals are a system for registering callbacks for specific events.
For example, if we press on a button, the "clicked" signal will be emitted.
The signal then takes care that all the registered callbacks will be executed.

`gtk-rs` provides convenience methods for registering callbacks.
In our "Hello World" example we [connected](../docs/gtk4/trait.ButtonExt.html#tymethod.connect_clicked) the "clicked" signal to a closure which sets the label of the button to "Hello World" as soon as it gets called.

<span class="filename">Filename: main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals/1/main.rs:callback}}
```

If we wanted to, we could have connected to it with the general (but much more verbose) `connect_local` method.

<span class="filename">Filename: main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals/2/main.rs:callback}}
```

The advantage of `connect_local` is, that it also works with custom signals.

Let us see how we can create our own signals.
Again, we do that by extending our `CustomButton`.
First we override the necessary methods in `ObjectImpl`.

<span class="filename">Filename: main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals/3/main.rs:object_impl}}
```

The `signal` method is responsible for defining a set of signals.
In our case, we only create a single signal named "max-number-reached".
When naming our signal, we make sure to do that in [kebab-case](https://wiki.c2.com/?KebabCase).
When emitted, it sends a single `i32` value and expects nothing in return.

We want the signal to be emitted, whenever `number` reaches `MAX_NUMBER`.
Together with the signal we send the value `number` currently holds.
After we did that, we set `number` back to 0.

<span class="filename">Filename: main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals/3/main.rs:button_impl}}
```

If we now press on the button, the number of its label increases until it reaches `MAX_NUMBER`.
Then it starts from 0 again and so on.

Custom signals are especially useful, if you want to notify consumers of your GObject that a certain event occurred.
