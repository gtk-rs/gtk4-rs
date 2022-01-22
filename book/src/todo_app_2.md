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
     <property name="width-request">360</property>
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
+            <property name="icon-name">open-menu-symbolic</property>
+            <property name="menu-model">main-menu</property>
+          </object>
+        </child>
+      </object>
+    </child>
     <child>
       <object class="GtkBox">
         <property name="orientation">vertical</property>
```

We also create a settings schema.
Again, the settings correspond to the stateful actions called by the menu.

<span class="filename">Filename: listings/todo_app/2/org.gtk-rs.Todo.gschema.xml</span>

```xml
{{#rustdoc_include ../listings/todo_app/2/org.gtk-rs.Todo.gschema.xml}}
```


Then we add `settings` and a reference to `clear_button` to `imp::Window`.
We stop deriving `Default` for `imp::Window` and implement it manually.

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

Similar to the previous chapter, we let `settings` create the action.
Then we add the newly created action "filter" to our window.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_filter_action}}
```

After activating the action "win.filter", the corresponding setting will be changed.
So we need a method which translates this setting into a filter that the `gtk::FilterListModel` understands.
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
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_model}}
```

In `setup_callbacks`, we add a signal handler to `clear_button`, which removes all completed tasks when activated.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_callbacks}}
```

In `setup_shortcut_window`, we add a handy way to let users of our app know which shortcuts they can use.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_shortcut_window}}
```

The entries can be organized with [`gtk::ShortcutsSection`](../docs/gtk4/struct.ShortcutsSection.html) and [`gtk::ShortcutsGroup`](../docs/gtk4/struct.ShortcutsGroup.html).
If we specify the action name, we also don't have to repeat the keyboard accelerator.
[`gtk::ShortcutsShortcut`](../docs/gtk4/struct.ShortcutsShortcut.html) looks it up on its own.
The `shortcuts.ui` file looks like this:

```xml
{{#rustdoc_include ../listings/todo_app/2/window/shortcuts.ui}}
```

> Note the way we set `action-name` for `ShortcutsShortcut`.
Instead of using a separate property for the target, it takes a *detailed* action name.
Detailed names look like this: `action_group.action_name(target)`.
Formatting of the target depends on its type and is documented [here](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Action.html#method.parse_detailed_name).
In particular, strings have to be enclosed single quotes as you can see in this example.


Finally, we bind the shortcuts to their actions with `set_accels_for_action`.
Here as well, a detailed action name is used.
Since this has to be done at the application level, `setup_shortcuts` takes a `gtk::Application` as parameter.

<span class="filename">Filename: listings/todo_app/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/main.rs:main}}
```

## Saving and Restoring Tasks

Since we use `Settings`, our filter state will persist between sessions.
However, the tasks themselves will not.
Let's implement that.

We could store our tasks in `Settings`, but it would be inconvenient.
When it comes to serializing and deserializing nothing beats the crate [`serde`](https://lib.rs/crates/serde).
Combined with [`serde_json`](https://lib.rs/crates/serde_json) we can save our tasks as serialized [json](https://en.wikipedia.org/wiki/JSON) files
First, we extend our `Cargo.toml` with the `serde` and `serde_json` crate.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Serde is a framework for serializing and deserializing Rust data structures.
The `derive` feature allows us to make our structures (de-)serializable with a single line of code.
We also use the `rc` feature so that Serde can deal with `std::rc::Rc` objects.

This is why we stored the data of `TodoObject` in a distinct `TodoData` structure.
Doing so allows us to derive `Serialize` and `Deserialize` for `TodoData`.

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:serialize}}
```

We plan to store our data as a file, so we create a utility function to provide a suitable file path for us.
We use [`glib::user_data_dir`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/fn.user_data_dir.html) to get the path to the config directory and create a new subdirectory for our app.
Then we return the file path.

<span class="filename">Filename: listings/todo_app/2/utils.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/utils.rs:data_path}}
```

We override the `close_request` virtual function to save the tasks when the window is closed.
To do so, we first iterate through all entries and store them in a `Vec`.
Then we serialize the `Vec` and store the data as a json file.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:window_impl}}
```

Note that we used [`serde_json::to_writer_pretty`](https://docs.serde.rs/serde_json/fn.to_writer_pretty.html) here.
The `pretty` suffix suggests that the json file is formatted in a readable way.
For your own app you might not care about this and go for [`serde_json::to_writer`](https://docs.serde.rs/serde_json/fn.to_writer.html) which produces smaller files.
For this example we like it, because it allows us to see into what a `Vec<TodoData>` will be serialized.

<span class="filename">Filename: data.json</span>

```json
[
  {
    "completed": true,
    "content": "Task Number Two"
  },
  {
    "completed": false,
    "content": "Task Number Five"
  },
  {
    "completed": true,
    "content": "Task Number Six"
  },
  {
    "completed": false,
    "content": "Task Number Seven"
  },
  {
    "completed": false,
    "content": "Task Number Eight"
  }
]
```

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
