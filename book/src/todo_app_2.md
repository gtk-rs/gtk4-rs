# Manipulating State of To-Do App

## Filtering Tasks

Now that we have learned how to use actions, we can continue working on our To-Do app.
One obvious feature to add would be filtering of tasks.
Let us use actions as entry points for this feature.
That way we can filter our task either via the menu or via keyboard accelerators.
We also add a button in the title bar, which removes all completed tasks when you click it.
This is how we want this to work in the end:

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="img/todo_app_2_animation.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

Let us start by adding the menu and title bar to `window.ui`.

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

We also add a reference to `clear_button` and add `settings` to `imp::Window`.
Since `gio::Settings` does not implement `Default`, we stop deriving `Default` for `imp::Window` and implement it manually.

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:struct_default}}
```

We also add the getter methods `is_completed` and `todo_data` to `TodoObject`.
These will be convenient later on.

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:impl}}
```

Similar to the chapter before, we let `settings` create the action.
Then we add the action "filter" to our window.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_filter_action}}
```

Before we move on to the logic, let us think about what we need.
After activating the action "win.filter", the setting will be changed.
So we need a method which can translate this setting state to a filter the `gtk::FilterListModel` can understand.
The possible states are "All", "Open" and "Done" and we return a filter only for "Open" and "Done".
If the state is "All" nothing has to be filtered out.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:filter}}
```

Now, we can setup the model.
We initialize `filter_model` with the value of the corresponding setting by calling the method `filter`.
Whenever the setting of key "filter" the changes, we call `filter` again to get the update `filter_model`.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_model}}
```

In `setup_callbacks` we add a signal handler to `clear_button`, which removes all done tasks when activated.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_callbacks}}
```

In `setup_shortcut_window` we add a shortcut window so that the users of our app know which shortcuts they can use.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_shortcut_window}}
```

Finally, we bind the shortcuts to their actions.
Since this has to be done at the application level, `setup_shortcuts` takes a `gtk::Application` as parameter.

<span class="filename">Filename: listings/todo_app/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/main.rs:main}}
```

## Saving and Restoring Tasks

Since we utilize `Settings` for handling our filter state it will persist between sesssions.
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
Because this allows us to derive `Serialize` and `Deserialize` on `TodoData`.

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:serialize}}
```

We plan to store our data as a file, so we create a utility function to provide a suitable path for us.
We use `glib::user_config_dir` to get the path to the config directory and create a new sub-directory for our app.
Then we return the final path.

<span class="filename">Filename: listings/todo_app/2/utils.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/utils.rs:data_path}}
```

We override the `close_request` virtual function to save the tasks whenever the window gets closed.
We iterate through all entries and store them in a `Vec`.
Then we serialize the `Vec` and store the data as a [JSON](https://en.wikipedia.org/wiki/JSON) file.


<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:window_impl}}
```

When we start the app, we will want to restore the saved data so let us add a `restore_data` method.
First, we make sure not to panic if there is no data file there yet.
It might be the first time that we started the app and in that case there is no former session to restore.

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:restore_data}}
```

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:object_impl}}
```
