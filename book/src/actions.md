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
We also connect a callback which closes the window.

<span class="filename">Filename: listings/actions/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:main}}
```

One of the most popular reasons to use actions are keyboard accelerators so we added one right now.
With [`set_accels_for_action`](../docs/gtk4/prelude/trait.GtkApplicationExt.html#tymethod.set_accels_for_action) one can assign one or more accelerators to a certain action.
Check the documentation of [`accelerator_parse`](../docs/gtk4/functions/fn.accelerator_parse.html) in order to learn more about its syntax.
Here we assigned `<primary>Q` which translates to `Ctrl+Q` on Linux and Windows or `Command+Q` on macOS.

Before we move on to other aspects of actions, let us appreciate a few things that are a few things curious here.
The "win" part of "win.quit" is the scope of the action.
But how does GTK know that "win" is the action scope of our window?
The answer is that it is so common to add actions to windows and applications that there are already two predefined scopes available:
- "app", for actions global to the application and
- "win", for actions tied to an application window.

If that would not be the case, we would have to add the action scope via [`gio::SimpleActionGroup`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.SimpleActionGroup.html).

<span class="filename">Filename: listings/actions/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/2/main.rs:action_group}}
```

Also, if we would have multiple windows we would expect that the currently focused window will be closed if we activate "win.quit".
And that what happens, but that also means that we actually define one action per window.
If we want to have a single globally accessible action instead, we add it to our application.

## Parameter & Stateful

Like most functions actions can take parameters.
However, unlike most functions they can also be stateful.
Let us see how that goes.

<span class="filename">Filename: listings/actions/4/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/4/main.rs:build_ui}}
```

<div style="text-align:center"><img src="img/actions_counter.gif" /></div>



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
