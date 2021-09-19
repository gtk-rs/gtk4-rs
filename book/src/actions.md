# Actions

By now, we have learned many ways already to glue our widgets together.
We can send messages through channels, emit signals, share reference-counted state and bind properties to each other.
Now, we will complete our set by learning about actions.

An action is a piece of functionality bound to a certain GObject.
Let us check out the most simple case where we activate an action without any parameters.

<span class="filename">Filename: listings/actions/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:build_ui}}
```

First, we create a new [`gio::SimpleAction`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SimpleAction.html) by specifying its name as "quit" and declaring that it takes no parameters.
As usual, we connect a callback where we then close the window.
Finally, we activate the action in the callback of the button.

There are a few things curious here.
The "win" part of "win.quit" is the scope of the action.
But how does GTK know that "win" is the action scope of our window?
Because it is common to add actions to windows and applications, there are two predefined scopes available:
- "app", for actions global to the application and
- "win", for actions tied to an application window.

If that would not be the case, we would have to add the action scope via [`gio::SimpleActionGroup`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SimpleActionGroup.html).

<span class="filename">Filename: listings/actions/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/2/main.rs:action_group}}
```

Also, if there are multiple windows and therefore potentially multiple actions named "win.quit", then how does GTK know which action to activate?
This is the reason why [`activate_action`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.WidgetExt.html#tymethod.activate_action) takes `&self` as first parameter.
`activate_action` up the action in the action groups associated with `self` and its ancestors.
This means that if we press the button belonging to window #3, the action of window #3 will be activated.
If we want to have a single globally accessible action instead, we add it to our application.

## Actionable

Connecting actions to the "clicked" signal of buttons is a typical use case, which is why all buttons implement the [Actionable](../docs/gtk4/struct.Actionable.html) interface.
That way the action can be specified by setting the property "action-name".
We can do that by simply adding one method call to our [`ButtonBuilder`](../docs/gtk4/struct.ButtonBuilder.html).

<span class="filename">Filename: listings/actions/3/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/3/main.rs:button_builder}}
```

This makes it also easily accessible through the interface builder.
As usual, we build up the window via a composite template.
Within the template we can then set the "action-name" property.

<span class="filename">Filename: listings/actions/4/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/actions/4/window/window.ui}}
```

The action can be added within `constructed`.

<span class="filename">Filename: listings/actions/4/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/4/window/imp.rs:object_impl}}
```

## Keyboard Accelerators

If you want to assign keyboard accelerators, actions are the way to go.
With [`set_accels_for_action`](../docs/gtk4/prelude/trait.GtkApplicationExt.html#tymethod.set_accels_for_action) one can assign one or more accelerators to a certain action.
Check the documentation of [`accelerator_parse`](../docs/gtk4/functions/fn.accelerator_parse.html) in order to learn more about its syntax.

For example, this how we assign Ctrl+Q (or Command+Q on macOS) to "win.quit".

<span class="filename">Filename: listings/actions/5/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/5/main.rs:main}}
```

# Menus
