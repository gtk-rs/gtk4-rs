# Manipulating State of To-Do App

## Filtering Tasks

Now it is time to continue working on our To-Do app.
One nice feature to add would be filtering of the tasks.
What a chance to use our newly gained knowledge of actions!
Using actions, we can access the filter via the menu as well as via keyboard shortcuts.
This is how we want this to work in the end:

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/todo_app_2_animation.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

Note that the screencast also shows a button with label "Clear" which will remove all done tasks.
This will come in handy when we later make the app preserve the tasks between sessions.

Let's start by adding a menu and a header bar to `window.ui`.
After reading the [actions](actions.html) chapter the added code should feel familiar.

<span class="filename">Filename: listings/todo_app/2/resources/window.ui</span>

```diff
 <?xml version="1.0" encoding="UTF-8"?>
 <interface>
+  <menu id="main-menu">
+    <submenu>
+      <attribute name="label" translatable="yes">_Filter</attribute>
+      <item>
+        <attribute name="label" translatable="yes">_All</attribute>
+        <attribute name="action">win.filter</attribute>
+        <attribute name="target">All</attribute>
+      </item>
+      <item>
+        <attribute name="label" translatable="yes">_Open</attribute>
+        <attribute name="action">win.filter</attribute>
+        <attribute name="target">Open</attribute>
+      </item>
+      <item>
+        <attribute name="label" translatable="yes">_Done</attribute>
+        <attribute name="action">win.filter</attribute>
+        <attribute name="target">Done</attribute>
+      </item>
+    </submenu>
+    <item>
+      <attribute name="label" translatable="yes">_Remove Done Tasks</attribute>
+      <attribute name="action">win.remove-done-tasks</attribute>
+    </item>
+    <item>
+      <attribute name="label" translatable="yes">_Keyboard Shortcuts</attribute>
+      <attribute name="action">win.show-help-overlay</attribute>
+    </item>
+  </menu>
   <template class="TodoWindow" parent="GtkApplicationWindow">
     <property name="width-request">360</property>
     <property name="title" translatable="yes">To-Do</property>
+    <child type="titlebar">
+      <object class="GtkHeaderBar">
+        <child type="end">
+          <object class="GtkMenuButton" id="menu_button">
+            <property name="icon-name">open-menu-symbolic</property>
+            <property name="menu-model">main-menu</property>
+          </object>
+        </child>
+      </object>
+    </child>
```

Then, we create a settings schema.
Again, the "filter" setting correspond to the stateful actions called by the menu.
We also add the "tasks" setting which is an array `a` of structs with a boolean `b` and string `s` as members (see [GVariant format string](https://docs.gtk.org/glib/gvariant-format-strings.html)).
We set the default to an empty array `[]` (see [GVariant Text Format](https://docs.gtk.org/glib/gvariant-text.html)).

<span class="filename">Filename: listings/todo_app/2/org.gtk-rs.Todo2.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/todo_app/2/org.gtk-rs.Todo2.gschema.xml}}
```

We install the schema as described in the settings [chapter](./settings.html)
Then we add a reference to `settings` and a reference to `clear_button` to `imp::Window`.
We stop deriving `Default` for `imp::Window` and implement it manually.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:struct_default}}
```

We also add the getter methods `is_completed` and `todo_data` to `TaskObject`.
We will make use of them in the following snippets.

<span class="filename">Filename: listings/todo_app/2/task_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/task_object/mod.rs:impl}}
```

Similar to the previous chapter, we let `settings` create the action.
Then we add the newly created action "filter" to our window.
We also add an action which allows us to remove done tasks. 

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_actions}}
```

After activating the action "win.filter", the corresponding setting will be changed.
So we need a method which translates this setting into a filter that the [`gtk::FilterListModel`](../docs/gtk4/struct.FilterListModel.html) understands.
The possible states are "All", "Open" and "Done". 
We return `Some(filter)` for "Open" and "Done".
If the state is "All" nothing has to be filtered out, so we return `None`.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:filter}}
```

Now, we can set up the model.
We initialize `filter_model` with the state from the settings by calling the method `filter`.
Whenever the state of the key "filter" changes, we call the method `filter` again to get the updated `filter_model`.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_tasks}}
```

Then, we bind the shortcuts to their actions with `set_accels_for_action`.
Here as well, a detailed action name is used.
Since this has to be done at the application level, `setup_shortcuts` takes a `gtk::Application` as parameter.

<span class="filename">Filename: listings/todo_app/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/main.rs:main}}
```

Now that we created all these nice shortcuts we will want our users to find them.
We do that by creating a shortcut window.
Again we use an `ui` file to describe it, but here we don't want to use it as template for our custom widget.
Instead we instantiate a widget of the existing class [`gtk::ShortcutsWindow`](../docs/gtk4/struct.ShortcutsWindow.html) with it. 


<span class="filename">Filename: listings/todo_app/2/resources/shortcuts.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/2/resources/shortcuts.ui}}
```

The entries can be organized with [`gtk::ShortcutsSection`](../docs/gtk4/struct.ShortcutsSection.html) and [`gtk::ShortcutsGroup`](../docs/gtk4/struct.ShortcutsGroup.html).
If we specify the action name, we also don't have to repeat the keyboard accelerator.
[`gtk::ShortcutsShortcut`](../docs/gtk4/struct.ShortcutsShortcut.html) looks it up on its own.


> Note the way we set `action-name` for `ShortcutsShortcut`.
Instead of using a separate property for the target, it takes a *detailed* action name.
Detailed names look like this: `action_group.action_name(target)`.
Formatting of the target depends on its type and is documented [here](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Action.html#method.parse_detailed_name).
In particular, strings have to be enclosed single quotes as you can see in this example.

Finally, we have to add the `shortcuts.ui` to our resources.
Note that we give it the alias `gtk/help-overlay.ui`.
We do that to take advantage of a convenience functionality documented [here](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Application.html#automatic-resources).
It will look for a resource at `gtk/help-overlay.ui` which defines a `ShortcutsWindow` with id `help_overlay`.
If it can find one it will create a action `win.show-help-overlay` which will show the window and associate the shortcut <kbd>Ctrl</kbd> + <kbd>?</kbd> with this action.

<span class="filename">Filename: listings/todo_app/2/resources/resources.gresource.xml</span>

```xml
{{#rustdoc_include ../listings/todo_app/2/resources/resources.gresource.xml}}
```

<div style="text-align:center"><img src="img/todo_app_2_shortcuts.png"/></div>


## Saving and Restoring Tasks

Our filter state will persist between sessions.
However, the tasks themselves will not.
Let's implement that.

Until now, we mostly converted primitives into variants.
However, the [`glib::Variant`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/derive.Variant.html) it also isn't much harder to do with our own structures.
This is why we stored the data of `TaskObject` in a distinct `TaskData` structure.
Doing so allows us to derive `glib::Variant` for `TaskData`.

<span class="filename">Filename: listings/todo_app/2/task_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/task_object/mod.rs:derive}}
```

We override the `close_request` virtual function to save the tasks when the window is closed.
To do so, we first iterate through all entries and store them in a `Vec`.
Then we serialize the `Vec` and store the data as a json file.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:window_impl}}
```

> Especially for more complicated types it isn't exactly easy to guess the correct variant text format.
> We can use this snippet to determine the text format of the type of `backup_data`.
> ```rust ,no_run,noplayground
> backup_data.to_variant().type_().as_str();
> ```

When we start the app, we will want to restore the saved data.
Let's add a `restore_data` method for that.
We make sure to handle the case where there is no data file there yet.
It might be the first time that we started the app and therefore there is no former session to restore.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:restore_data}}
```

Finally, we make sure that everything is set up in `constructed`.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:object_impl}}
```

Our To-Do app suddenly became much more useful.
Not only can we filter tasks, we also retain our tasks between sessions.
