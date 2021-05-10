# Saving Window State

Quite often, we want the window state to persist between sessions.
If the user resizes or maximizes the window, they might expect to find it in the same state the next time they open the app.
GTK does not provide this functionality out of the box, but luckily it is not too hard to manually implement it.
We basically want two integers (`height` & `width`) and a boolean (`is_maximized`) to persist.
We already know how to do this by using `Settings`.

<span class="filename">Filename: listings/saving_window_state/1/org.gtk.example.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/saving_window_state/1/org.gtk.example.gschema.xml}}
```

Since we do not care about intermediate state, we only load the window state when the window is constructed and save it when we close the window.
That can be done by creating a custom window.
First, we create one and add methods for getting and setting the window state.

<span class="filename">Filename: listings/saving_window_state/1/custom_window/mod.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/saving_window_state/1/custom_window/mod.rs:mod}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

The implementation struct holds the `settings`.
We also overload the `constructed` and `close_request` methods, where we load or save the window state. 

<span class="filename">Filename: listings/saving_window_state/1/custom_window/imp.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/saving_window_state/1/custom_window/imp.rs:imp}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

That is it!
Now our window retains its state between app sessions.

Please note how we handle a failure in saving into the settings.
We do not want to panic for recoverable errors.
We might also not want to present all problems at the GUI.
In our case we could not even do this, because the window will be immediately closed after the error occurs.
Logging is the standard way of handling a situation like this.
For that, we need to add the `log` crate and one of its front-ends, such as `pretty_env_logger`, to our dependencies.

<span class="filename">Filename: listings/Cargo.toml</span>

```toml
[dependencies]
log = "0.4"
pretty_env_logger = "0.4"
```

We then have to initialize `pretty_env_logger` by calling `init` in `main`.

<span class="filename">Filename: listings/saving_window_state/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/saving_window_state/1/main.rs:main}}
```

We can now modify the log level by setting the `RUST_LOG` environment variable as can be seen [here](https://docs.rs/env_logger/latest/env_logger/)
