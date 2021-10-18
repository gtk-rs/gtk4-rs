# Manipulating State of To-Do App

## Filtering Tasks

Now it is time to continue working on our To-Do app.
One nice feature to add would be to allow filtering of tasks.
What a chance to use our newly gained knowledge of actions!
Using actions, we can access the filter via the menu as well as via keyboard accelerators.
This is how we want this to work in the end:

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="img/todo_app_2_animation.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

Note that the screencast also shows a `clear_button` which will remove all done tasks.
This will come in handy when we later allow the app to keep its tasks between sessions.

Let us start by adding the menu and title bar to `window.ui`.
The changed code should feel familiar to the one in the former chapter.

<span class="filename">Filename: listings/todo_app/2/window/window.ui</span>

```diff
 <?xml version="1.0" encoding="UTF-8"?>
 <interface>
+  <menu id="main-menu">
+    <submenu>
+      <attribute name="label" translatable="yes">_Filtering</attribute>
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
+      <attribute name="label" translatable="yes">_Keyboard Shortcuts</attribute>
+      <attribute name="action">win.show-help-overlay</attribute>
+    </item>
+  </menu>
   <template class="TodoWindow" parent="GtkApplicationWindow">
     <property name="width_request">360</property>
     <property name="title" translatable="yes">To-Do</property>
+    <child type="titlebar">
+      <object class="GtkHeaderBar">
+        <child type="start">
+          <object class="GtkButton" id="clear_button">
+            <property name="label">Clear</property>
+          </object>
+        </child>
+        <child type ="end">
+          <object class="GtkMenuButton" id="menu_button">
+            <property name="icon_name">open-menu-symbolic</property>
+            <property name="menu_model">main-menu</property>
+          </object>
+        </child>
+      </object>
+    </child>
     <child>
       <object class="GtkBox">
         <property name="orientation">vertical</property>
```

We also add `settings` and a reference to `clear_button` to `imp::Window`.
Since `gio::Settings` does not implement `Default`, we stop deriving `Default` for `imp::Window` and implement it manually.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:struct_default}}
```

We also add the getter methods `is_completed` and `todo_data` to `TodoObject`.
We will make use of them in the following snippets.

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:impl}}
```

Similar to the chapter before, we let `settings` create the action.
Then we add the newly created action "filter" to our window.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_filter_action}}
```

After activating the action "win.filter", the corresponding setting will be changed.
So we need a method which translates this setting to a filter the `gtk::FilterListModel` understands.
The possible states are "All", "Open" and "Done" and we return a `Some(filter)` for "Open" and "Done".
If the state is "All" nothing has to be filtered out so we return `None`.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:filter}}
```

Now, we can setup the model.
We initialize `filter_model` with the setting state by calling the method `filter`.
Whenever the setting of the key "filter" changes, we call the method `filter` again to get the updated `filter_model`.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_model}}
```

In `setup_callbacks` we add a signal handler to `clear_button`, which removes all done tasks when activated.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_callbacks}}
```

In `setup_shortcut_window` we add a handy way to let users of our app know which shortcuts they can use.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_shortcut_window}}
```

The entries can be organized with [`gtk::ShortcutsSection`](../docs/gtk4/struct.ShortcutsSection.html) and [`gtk::ShortcutsGroup`](../docs/gtk4/struct.ShortcutsGroup.html).
If we specify the action name, we also do not have to repeat the keyboard accelerator.
[`gtk::ShortcutsShortcut`](../docs/gtk4/struct.ShortcutsShortcut.html) looks it up on its own.
The `shortcuts.ui` file looks like this:

```xml
{{#rustdoc_include ../listings/todo_app/2/window/shortcuts.ui}}
```

> Note the way we specified the target for `ShortcutsShortcut` and later `set_accels_for_action`.
Instead of accepting a separate property for the target, it takes a *detailed* action name.
It follows the following system: `action_group.action_name(target)`.
The way the target has to be formatted depends on the type of the target and is documented [here](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Action.html#method.parse_detailed_name).


Finally, we bind the shortcuts to their actions with `set_accels_for_action`.
Since this has to be done at the application level, `setup_shortcuts` takes a `gtk::Application` as parameter.

<span class="filename">Filename: listings/todo_app/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/main.rs:main}}
```

## Saving and Restoring Tasks

Since we utilize `Settings`, our filter state will persist between sessions.
Unfortunately, the same cannot be said about the actual tasks.
Let us change that.

First we extend our `Cargo.toml` with the popular [`serde`](https://lib.rs/crates/serde) and corresponding [`serde_json`](https://lib.rs/crates/serde_json) crate.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive", "rc"] }
serde_json = "1.0"
```

Serde is a framework for serializing and deserializing Rust data structures.
Activating the `derive` feature allows us to derive the necessary traits for this automatically.
We also use the `rc` feature so that Serde can deal with `std::rc::Rc` objects.

Now it should be clear why the data of `TodoObject` is stored in a distinct `TodoData` object.
Doing so allows us to derive `Serialize` and `Deserialize` for `TodoData`.

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:serialize}}
```

We plan to store our data as a file, so we create a utility function to provide a suitable file path for us.
We use [`glib::user_config_dir`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/fn.user_config_dir.html) to get the path to the config directory and create a new subdirectory for our app.
Then we return the file path.

<span class="filename">Filename: listings/todo_app/2/utils.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/utils.rs:data_path}}
```

We override the `close_request` virtual function to save the tasks whenever the window gets closed.
To do so, we first iterate through all entries and store them in a `Vec`.
Then we serialize the `Vec` and store the data as a [JSON](https://en.wikipedia.org/wiki/JSON) file.


<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:window_impl}}
```

When we start the app, we will want to restore the saved data.
Let us add a `restore_data` method for that.
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

Our app suddenly became much more useful.
Not only can we filter tasks, we also retain our tasks between session.
