# Settings

We have now learned multiple ways to handle states.
However, every time we close the application all of it is gone.
Let's learn how to use [`gio::Settings`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Settings.html) by storing the state of a [`Switch`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Switch.html) in it.

At the very beginning we have to create a `GSchema` xml file in order to describe the kind of data our application plans to store in the settings.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/1/org.gtk_rs.Settings1.gschema.xml">listings/settings/1/org.gtk_rs.Settings1.gschema.xml</a>

```xml
{{#rustdoc_include ../listings/settings/1/org.gtk_rs.Settings1.gschema.xml}}
```
Let's get through it step by step.
The `id` is the same application id we used when we created our application.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/1/main.rs">listings/settings/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/settings/1/main.rs:application}}
```
The `path` must start and end with a forward slash character ('/') and must not contain two sequential slash characters.
When creating a `path`, we advise to take the `id`, replace the '.' with '/' and add '/' at the front and end of it.

We only want to store a single key with the `name` "is-switch-enabled".
This is a boolean value so its `type` is "b" (see [GVariant Format Strings](https://docs.gtk.org/glib/gvariant-format-strings.html) for the other options).
We also set its default value to `false` (see [GVariant Text Format](https://docs.gtk.org/glib/gvariant-text.html) for the full syntax).
Finally, we add a summary.

Now we need to copy and compile the schema.

> You can install the schema by executing the following commands on a Linux or macOS machine:
> ```bash
> mkdir -p $HOME/.local/share/glib-2.0/schemas
> cp org.gtk_rs.Settings1.gschema.xml $HOME/.local/share/glib-2.0/schemas/
> glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
> ```
> 
> On Windows run:
> ```powershell
> mkdir C:/ProgramData/glib-2.0/schemas/
> cp org.gtk_rs.Settings1.gschema.xml C:/ProgramData/glib-2.0/schemas/
> glib-compile-schemas C:/ProgramData/glib-2.0/schemas/
> ```

We initialize the `Settings` object by specifying the application id.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/1/main.rs">listings/settings/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/settings/1/main.rs:settings}}
```

Then we get the settings key and use it when we create our `Switch`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/1/main.rs">listings/settings/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/settings/1/main.rs:switch}}
```

Finally, we assure that the switch state is stored in the settings whenever we click on it.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/1/main.rs">listings/settings/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/settings/1/main.rs:connect_state_set}}
```

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/settings_1.webm" type="video/webm">
  <p>A video which shows that the app can now store the app state</p>
 </video>
</div>

The `Switch` now retains its state even after closing the application.
But we can make this even better.
The `Switch` has a property "active" and `Settings` allows us to bind properties to a specific setting.
So let's do exactly that.

We can remove the [`boolean`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/prelude/trait.SettingsExt.html#tymethod.boolean) call before initializing the `Switch` as well as the [`connect_state_set`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Switch.html#method.connect_state_set) call.
We then bind the setting to the property by specifying the key, object and name of the property.
Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/settings/2/main.rs">listings/settings/2/main.rs</a>

```rust
{{#rustdoc_include ../listings/settings/2/main.rs:settings_bind}}
```

Whenever you have a property which nicely correspond to a setting, you probably want to bind it to it.
In other cases, interacting with the settings via the getter and setter methods tends to be the right choice.
