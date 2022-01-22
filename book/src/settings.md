# Settings

We have now learned multiple ways to handle states.
However, every time we close the application all of it is gone.
Let's learn how to use `GSettings` by storing the state of a [`Switch`](../docs/gtk4/struct.Switch.html) in it.

At the very beginning we have to create a `GSchema` xml file in order to describe the kind of data our application plans to store in the settings.

<span class="filename">Filename: listings/settings/1/org.gtk-rs.example.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/settings/1/org.gtk-rs.example.gschema.xml}}
```
Let's get through it step by step.
The `id` is the same application id we used when we created our application.

<span class="filename">Filename: listings/settings/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/settings/1/main.rs:application}}
```
The `path` must start and end with a forward slash character ('/') and must not contain two sequential slash characters.
When creating a `path`, we advise to take the `id`, replace the '.' with '/' and add '/' at the front and end of it.

We only want to store a single key with the `name` "is-switch-enabled".
This is a boolean value so its `type` is "b" (see [GVariant Format Strings](https://docs.gtk.org/glib/gvariant-format-strings.html) for the other options).
Finally, we define its default value and add a summary.

Now we need to copy and compile the schema.

You can install the schema by executing the following commands on a Linux or macOS machine:
```bash
sudo cp org.gtk-rs.example.gschema.xml /usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
```

On Windows run:
```powershell
cp org.gtk-rs.example.gschema.xml C:/ProgramData/glib-2.0/schemas/
glib-compile-schemas C:/ProgramData/glib-2.0/schemas/
```

We initialize the `Settings` object by specifying the application id.

<span class="filename">Filename: listings/settings/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/settings/1/main.rs:settings}}
```

Then we get the settings key and use it when we create our `Switch`.

<span class="filename">Filename: listings/settings/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/settings/1/main.rs:switch}}
```

Finally, we assure that the switch state is stored in the settings whenever we click on it.

<span class="filename">Filename: listings/settings/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/settings/1/main.rs:connect_state_set}}
```

The `Switch` now retains its state even after closing the application.
But we can make this even better.
The `Switch` has a property "state" and `Settings` allows us to bind properties to a specific setting.
So let's do exactly that.

We can remove the [`boolean`](http://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/trait.SettingsExt.html#tymethod.boolean) call before initializing the `Switch` as well as the `connect_state_set` call.
We then bind the setting to the property by specifying the key, object and name of the property.
Additionally, we specify [`SettingsBindFlags`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SettingsBindFlags.html) to control the direction in which the binding works.

<span class="filename">Filename: listings/settings/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/settings/2/main.rs:settings_bind}}
```

Whenever you have a property which nicely correspond to a setting, you probably want to bind it to it.
In other cases, interacting with the settings via the getter and setter methods tends to be the right choice.
