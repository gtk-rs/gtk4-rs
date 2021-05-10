# Signals

GObject signals are a system for registering callbacks for specific events.
For example, if we press on a button, the "clicked" signal will be emitted.
The signal then takes care that all the registered callbacks will be executed.

`gtk-rs` provides convenience methods for registering callbacks.
In our "Hello World" example we [connected](../docs/gtk4/trait.ButtonExt.html#tymethod.connect_clicked) the "clicked" signal to a closure which sets the label of the button to "Hello World" as soon as it gets called.

<span class="filename">Filename: listings/gobject_signals/1/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/gobject_signals/1/main.rs:callback}}
```

If we wanted to, we could have 
connected to it with the generic [`connect_local`](http://gtk-rs.org/docs/glib/object/trait.ObjectExt.html#tymethod.connect_local) method.

<span class="filename">Filename: listings/gobject_signals/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_signals/2/main.rs:callback}}
```

Similar to the generic way of accessing properties, the advantage of `connect_local` is that it also works with custom signals[^1].

Let us see how we can create our own signals.
Again, we do that by extending our `CustomButton`.
First we override the necessary methods in `ObjectImpl`.

<span class="filename">Filename: listings/gobject_signals/3/custom_button/imp.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_signals/3/custom_button/imp.rs:object_impl}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

The `signal` method is responsible for defining a set of signals.
In our case, we only create a single signal named "max-number-reached".
When naming our signal, we make sure to do that in [kebab-case](https://wiki.c2.com/?KebabCase).
When emitted, it sends a single `i32` value and expects nothing in return.

We want the signal to be emitted, whenever `number` reaches `MAX_NUMBER`.
Together with the signal we send the value `number` currently holds.
After we did that, we set `number` back to 0.

<span class="filename">Filename: listings/gobject_signals/3/custom_button/imp.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_signals/3/custom_button/imp.rs:button_impl}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

If we now press on the button, the number of its label increases until it reaches `MAX_NUMBER`.
Then it emits the "max-number-reached" signal which we can nicely connect to.
Whenever we now receive the "max-number-reached" signal, the accompanying number is printed to standard output.

<span class="filename">Filename: listings/gobject_signals/3/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_signals/3/main.rs:signal_handling}}
```

You now know how to connect to every kind of signal and how to create your own.
Custom signals are especially useful, if you want to notify consumers of your GObject that a certain event occurred.


[^1]: If you want to connect from a different thread than the main thread, make sure to use [`connect`](http://gtk-rs.org/docs/glib/object/trait.ObjectExt.html#tymethod.connect) instead of `connect_local`. However, that also means that your connected closure has to implement [`Send`](https://doc.rust-lang.org/stable/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/stable/core/marker/trait.Sync.html).
