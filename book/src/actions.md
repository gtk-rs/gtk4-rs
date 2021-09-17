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

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/actions/2/main.rs:action_group}}
```

Also, if there are multiple windows and therefore potentially multiple actions named "win.quit", then how does GTK know which action to activate?
This is the reason why [`activate_action`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.WidgetExt.html#tymethod.activate_action) takes `&self` as first parameter.
`activate_action` up the action in the action groups associated with `self` and its ancestors.
This means that if we press the button belonging to window #3, the action of window #3 will be activated.
If we want to have a single globally accessible action instead, we add it to our application instead.




## Keybindings

# Menus
