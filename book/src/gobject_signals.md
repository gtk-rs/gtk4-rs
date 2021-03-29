# Signals

GObject signals are a system for registering callbacks for specific events.
For example, if we press on a button, the "clicked" signal will be emitted.
We only need to connect a callback to the "clicked" signal and it will be called whenever the button has be clicked on.
`gtk-rs` provides convenience methods for all widgets to do exactly that.
In our "Hello World" example we connected the "clicked" signal to a closure which sets the label of the button to "Hello World" as soon as it gets called.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals_1/src/main.rs:callback}}
```

If we wanted to, we could have connected to it with the general (but much more verbose) method.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals_2/src/main.rs:callback}}
```

For pre-existing GObjects, we do not want to do that.
However, if we have custom objects, we might also add custom signals to it.
So let us see how to do that.

First, we add `once_cell` to our dependencies.
With this, we can [lazily evaluate](https://en.wikipedia.org/wiki/Lazy_evaluation) expressions, which we often need when working with custom GObjects.

```toml
[dependencies]
once_cell = "1"
```

Let us revive the `CustomButton` of the last section and teach it a few new tricks.
First we override the necessary methods in `ObjectImpl`.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals_3/src/main.rs:object_impl}}
```

The `signal` method is responsible for defining a set of signals.
We only create a single signal named "max-number-reached".
When emitted, it sends a single `i32` value and expects nothing in return.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals_3/src/main.rs:button_impl}}
```

We want the signal to be emitted, whenever `number` reached `MAX_NUMBER`.
Together with the signal we send the value `number` currently holds.
After we did that, we set `number` back to 0.


Signals are especially useful, if you want to notify consumers of your GObject that a certain event occurred.