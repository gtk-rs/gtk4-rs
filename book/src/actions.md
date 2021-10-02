# Actions

By now, we have already learned many ways to glue our widgets together.
We can send messages through channels, emit signals, share reference-counted state and bind properties.
Now, we will complete our set by learning about actions.

An action is a piece of functionality bound to a certain GObject.
Let us check out the simplest case where we activate an action without a parameter.

<span class="filename">Filename: listings/actions/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:build_ui}}
```

First, we created a new [`gio::SimpleAction`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SimpleAction.html) which is named "quit" and takes no parameter.
We also connected a callback which closes the window.

<span class="filename">Filename: listings/actions/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:main}}
```

One of the most popular reasons to use actions are keyboard accelerators, so we added one here.
With [`set_accels_for_action`](../docs/gtk4/prelude/trait.GtkApplicationExt.html#tymethod.set_accels_for_action) one can assign one or more accelerators to a certain action.
Check the documentation of [`accelerator_parse`](../docs/gtk4/functions/fn.accelerator_parse.html) in order to learn more about its syntax.
Here we assigned `<primary>W` which translates to <kbd>Ctrl</kbd> + <kbd>W</kbd> on Linux and Windows and ⌘ + <kbd>W</kbd> on macOS.

Before we move on to other aspects of actions, let us appreciate a few things that are curious here.
The "win" part of "win.quit" is the group of the action.
But how does GTK know that "win" is the action group of our window?
The answer is that it is so common to add actions to windows and applications that there are already two predefined groups available:
- "app" for actions global to the application, and
- "win" for actions tied to an application window.

If that was not the case, we would have to add the action group manually via [`gio::SimpleActionGroup`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SimpleActionGroup.html).

<span class="filename">Filename: listings/actions/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/2/main.rs:action_group}}
```

Also, if we had multiple instances of the same windows we would expect that only the currently focused window will be closed when activating "win.quit".
And indeed, "win.quit" on the Application dispatches the action to the currently focused window.
However, that also means that we actually define one action per window instance.
If we want to have a single globally accessible action instead, we add it to our application.

## Parameter and State

An action, like most functions, can take a parameter.
However, unlike most functions it can also be stateful.
Let us see how this works.

<span class="filename">Filename: listings/actions/3/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/3/main.rs:build_ui}}
```

Here we created a "win.count" action that increases its state by the given parameter every time it is activated.
It also takes care of updating the `label` with the current state.
The button activates the action with each click while passing "1" as parameter.
This is how our app works:

<div style="text-align:center"><img src="img/actions_counter.gif" /></div>


## Actionable

Connecting actions to the "clicked" signal of buttons is a typical use case, which is why all buttons implement the [`Actionable`](../docs/gtk4/struct.Actionable.html) interface.
This way, the action can be specified by setting the "action-name" property.
If the action accepts a parameter, it can be set via the "action-target" property.
With [`ButtonBuilder`](../docs/gtk4/struct.ButtonBuilder.html), we can set everything up by calling its methods.

<span class="filename">Filename: listings/actions/4/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/4/main.rs:button_builder}}
```

Actionable widgets are also easily accessible through the interface builder.
As usual, we build up the window via a composite template.
Within the template we can then set the "action-name" and "action-target" properties.

<span class="filename">Filename: listings/actions/5/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/actions/5/window/window.ui}}
```

We will connect the actions and add them to the window in the `Window::add_actions` method.

<span class="filename">Filename: listings/actions/5/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/5/window/mod.rs:impl_window}}
```

Finally, `add_actions` will be called within `constructed`.

<span class="filename">Filename: listings/actions/5/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/5/window/imp.rs:object_impl}}
```

This app behaves the same as our previous example, but it leads us to menu creation.


## Menus

If you want to create a [menu](https://developer.gnome.org/hig/patterns/controls/menus.html) you have to use actions and you will want to use the interface builder.
Typically, a menu entry has an action fitting one of these three descriptions:
- no parameter and no state, or
- no parameter and boolean state, or
- string parameter and string state.

Let us modify our small app to demonstrate these cases.
First we extend `add_actions`.
For the action without parameter or state, we choose "win.quit" which we are already familiar with.


<span class="filename">Filename: listings/actions/6/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/6/window/mod.rs:action_quit}}
```
With the action "sensitive-button", we manipulate the "sensitive" property of `button`.
We need to be careful to follow the established conventions: actions with no parameter and boolean state should behave like toggle actions.
This means that the caller can expect the boolean state to toggle after activating the action.

<span class="filename">Filename: listings/actions/6/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/6/window/mod.rs:action_sensitive_button}}
```

Finally, we add "win.orientation", an action with string parameter and string state.
This action can be used to change the orientation of `gtk_box`.
Here the convention is that the state should be set to the given parameter.
The logic of the code does not rely on a state but with it, the menu will be able to display the current state.

<span class="filename">Filename: listings/actions/6/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/6/window/mod.rs:action_orientation}}
```

Even though [`gio::Menu`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Menu.html) can also be created with the bindings, the most convenient way is to use the interface builder for that.
We do that by adding the menu in front of the template.

<span class="filename">Filename: listings/actions/6/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/actions/6/window/window.ui}}
```

While it is less readable, we could have also 
Note how we specified a target:

```xml
<attribute name="target">Horizontal</attribute>
```

String is the default type of the target which is why we did not have to specify a type.
With targets of other types you need to manually specify the correct [GVariant format string](https://docs.gtk.org/glib/gvariant-format-strings.html).
For example, an `i32` variable with value "5" would correspond to this:

```xml
<attribute name="target" type="i">5</attribute>
```

This is how the app looks in action:

<div style="text-align:center"><img src="img/actions_menu.gif" /></div>


## Settings

The menu entries nicely display the state of our stateful actions, but after the app is closed all changes to that state are lost.
As usual, we solve this problem with `gio::Settings`.
First we add the settings to `imp::Window` and manually implement the `Default` trait.

<span class="filename">Filename: listings/actions/7/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/7/window/imp.rs:imp_struct}}
```

Creating stateful actions from setting entries is so common that `Settings` provides a method for that exact purpose.
We create actions with `create_actions` and then add them to the action group of our window.

<span class="filename">Filename: listings/actions/7/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/7/window/mod.rs:settings_create_actions}}
```

Since actions from `create_actions` follow the aforementioned conventions, we can keep further changes to a minimum.
The action "win.sensitive-button" toggles its state with each activation and the state of the "win.orientation" action follows the given parameter.

We still have to specify what should happen when the actions are activated though.
For the stateful actions, instead of adding callbacks to their "activate" signals we bind the settings to properties we want to manipulate.

<span class="filename">Filename: listings/actions/7/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/7/window/mod.rs:bind_settings}}
```

Finally, we make sure that `bind_settings` is called within `constructed`.

<span class="filename">Filename: listings/actions/7/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/7/window/imp.rs:object_impl}}
```

Actions are extremely powerful and we are only scratching the surface here.
If you want to learn more about them, the [GNOME developer documentation](https://developer.gnome.org/documentation/tutorials/actions.html) is a good place to start.
