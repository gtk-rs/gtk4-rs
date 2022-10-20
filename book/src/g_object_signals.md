# Signals

GObject signals are a system for registering callbacks for specific events.
For example, if we press on a button, the "clicked" signal will be emitted.
The signal then takes care that all the registered callbacks will be executed.

`gtk-rs` provides convenience methods for registering callbacks.
In our "Hello World" example we [connected](../docs/gtk4/prelude/trait.ButtonExt.html#tymethod.connect_clicked) the "clicked" signal to a closure which sets the label of the button to "Hello World" as soon as it gets called.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/hello_world/3/main.rs">listings/hello_world/3/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/3/main.rs:callback}}
```

If we wanted to, we could have connected to it with the generic [`connect_closure`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/object/trait.ObjectExt.html#tymethod.connect_closure) method and the [`glib::closure_local!`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.closure_local.html) macro.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_signals/1/main.rs">listings/g_object_signals/1/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_signals/1/main.rs:callback}}
```

Similar to the generic way of accessing properties, the advantage of `connect_closure` is that it also works with custom signals.

> If you need to clone reference counted objects into your closure you don't have to wrap it within another `clone!` macro.
> `closure_local!` accepts the same syntax for creating strong/weak references, plus a [watch](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.closure.html#object-watching) feature that automatically disconnects the closure once the watched object is dropped. 

## Adding Signals to Custom GObjects

Let's see how we can create our own signals.
Again we do that by extending our `CustomButton`.
First we override the necessary methods in `ObjectImpl`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_signals/2/custom_button/imp.rs">listings/g_object_signals/2/custom_button/imp.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_signals/2/custom_button/imp.rs:object_impl}}
```

The `signals` method is responsible for defining a set of signals.
In our case, we only create a single signal named "max-number-reached".
When naming our signal, we make sure to do that in [kebab-case](https://en.wikipedia.org/wiki/Letter_case#Kebab_case).
When emitted, it sends a single `i32` value.

We want the signal to be emitted, whenever `number` reaches `MAX_NUMBER`.
Together with the signal we send the value `number` currently holds.
After we did that, we set `number` back to 0.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_signals/2/custom_button/imp.rs">listings/g_object_signals/2/custom_button/imp.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_signals/2/custom_button/imp.rs:button_impl}}
```

If we now press on the button, the number of its label increases until it reaches `MAX_NUMBER`.
Then it emits the "max-number-reached" signal which we can nicely connect to.
Whenever we now receive the "max-number-reached" signal, the accompanying number is printed to [standard output](https://en.wikipedia.org/wiki/Standard_streams#Standard_output_(stdout)).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/g_object_signals/2/main.rs">listings/g_object_signals/2/main.rs</a>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/g_object_signals/2/main.rs:signal_handling}}
```

You now know how to connect to every kind of signal and how to create your own.
Custom signals are especially useful, if you want to notify consumers of your GObject that a certain event occurred.
