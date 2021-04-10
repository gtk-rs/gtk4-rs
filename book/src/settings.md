# Settings

We have now learned multiple ways to handle states.
However, every time we close the application all of it is gone.
Let us learn how to use `GSettings` by storing the state of a [`Switch`](../docs/gtk4/struct.Switch.html) in it.

At the very beginning we have to create a `GSchema` xml file in order to describe the kind of data our application plans to store in the settings.

<span class="filename">Filename: org.gtk.example.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/settings/1/org.gtk.example.gschema.xml}}
```
Let us get through it step by step.
The `id` is the same application id we used when we created our application.

<span class="filename">Filename: main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings/1/main.rs:application}}
```
The `path` must start and end with a forward slash character ('/') and must not contain two sequential slash characters.
When creating a `path`, we advise to take the `id`, replace the '.' with '/' and add '/' at the front and end of it.

We only want to store a single key with the `name` "is-switch-enabled".
This is a boolean value so its `type` is "b" (see [GVariant Format Strings](https://developer.gnome.org/glib/stable/gvariant-format-strings.html) for the other options).
Finally, we define its default value and add a summary.

Now we have to install the `GSchema` by executing the following commands:

```bash
$ sudo install -D org.gtk.example.gschema.xml /usr/share/glib-2.0/schemas/
$ sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

This has to be repeated every time we change the `GSchema`.
That is why we probably want to use a build system like [Meson](https://mesonbuild.com/) to do it for us.

We initialize the `Settings` object by specifying the application id.

<span class="filename">Filename: main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings/1/main.rs:settings}}
```

Then we get the settings key and use it when we create our `Switch`.

<span class="filename">Filename: main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings/1/main.rs:switch}}
```

Finally, we assure that the switch state is stored in the settings whenever we click on it.

<span class="filename">Filename: main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings/1/main.rs:connect_state_set}}
```

The `Switch` now retains its state even after closing the application.
But we can make this even better.
The `Switch` has a property "state" and `Settings` allows us to bind properties to a specific setting.
So let us do exactly that.

We can remove the `get_boolean` call before initializing the `Switch` as well as the `connect_state_set` call.
We then bind the setting to the property by specifying the key, object and name of the property.
We also specify [`SettingsBindFlags`](https://gtk-rs.org/docs/gio/struct.SettingsBindFlags.html) to control the direction in which the binding works.

<span class="filename">Filename: main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings/2/main.rs:settings_bind}}
```

Whenever you have a property which nicely correspond to a setting, you probably want to bind it to it.
In other cases, interacting with the settings via the getter and setter methods tends to be the right choice.
