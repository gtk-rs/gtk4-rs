# Settings

We have now learned multiple ways to handle state.
However, every time we close the application all of it is gone.
Let's learn how to use `GSettings` by storing the state of a [`Switch`](https://gtk-rs.org/gtk4-rs/gtk4/struct.Switch.html) in it.

<div style="text-align:center"><img src="img/settings_buttons.png" /></div>

At the very beginning we have to create a `GSchema` xml file in order to describe the kind of data our application plans to store in the settings.

<span class="filename">Filename: org.gtk.example.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/settings_1/org.gtk.example.gschema.xml}}
```
Let's get through it step by step.
The `id` is the same application id we used when we created our application.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings_1/src/main.rs:application}}
```
The `path` must start and end with a forward slash character ('/') and must not contain two sequential slash characters.
When creating a `path`, we advise to take the `id`, replace the '.' with '/' and add '/' at the front and end of it.

We only want to store a single key with the `name` “is-switch-enabled”.
This is a boolean value so its `type` is “b” (see [GVariant Format Strings](https://developer.gnome.org/glib/stable/gvariant-format-strings.html) for the other options).
Finally, we define its default value and add a summary.

Now we have to install the `GSchema` by executing the following commands:

```bash
$ sudo install -D org.gtk.example.gschema.xml /usr/share/glib-2.0/schemas/
$ sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

This has to be repeated every time we change the `GSchema`.
That is why we probably want to use a build system like [Meson](https://mesonbuild.com/) to do it for us.

We initialize the `Settings` object by specifying the application id.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings_1/src/main.rs:settings}}
```

Then we get the settings key and use it when we create our `Switch`.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings_1/src/main.rs:switch}}
```

Finally, we assure that the switch state is stored in the settings whenever we click on it.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings_1/src/main.rs:connect_state_set}}
```

We have now written a small application which retains its switch state even after closing it.