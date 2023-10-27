# Actions

By now, we've already learned many ways to glue our widgets together.
We can send messages through channels, emit signals, share reference-counted state and bind properties.
Now, we will complete our set by learning about actions.

An action is a piece of functionality bound to a certain GObject.
Let's check out the simplest case where we activate an action without a parameter.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/1/main.rs">listings/actions/1/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:build_ui}}
```

First, we created a new [`gio::ActionEntry`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.ActionEntry.html) which is named "close" and takes no parameter.
We also connected a callback which closes the window when the action is activated.
Finally, we add the action entry to the window via [`add_action_entries`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/prelude/trait.ActionMapExtManual.html#method.add_action_entries).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/1/main.rs">listings/actions/1/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/actions/1/main.rs:main}}
```

One of the most popular reasons to use actions are keyboard accelerators, so we added one here.
With [`set_accels_for_action`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.GtkApplicationExt.html#tymethod.set_accels_for_action) one can assign one or more accelerators to a certain action.
Check the documentation of [`accelerator_parse`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/functions/fn.accelerator_parse.html) in order to learn more about its syntax.

Before we move on to other aspects of actions, let's appreciate a few things that are curious here.
The "win" part of "win.close" is the group of the action.
But how does GTK know that "win" is the action group of our window?
The answer is that it is so common to add actions to windows and applications that there are already two predefined groups available:
- "app" for actions global to the application, and
- "win" for actions tied to an application window.

We can add a action group to any widget via the method [`insert_action_group`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.WidgetExt.html#method.insert_action_group).
Let's add our action to the action group "custom-group" and add the group then to our window.
The action entry isn't specific to our window anymore, the first parameter of the "activate" callback is of type `SimpleActionGroup` instead of `ApplicationWindow`.
This means we have to clone `window` into the closure.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/2/main.rs">listings/actions/2/main.rs</a>

```rust
{{#rustdoc_include ../listings/actions/2/main.rs:build_ui}}
```

If we bind the accelerator to "custom-group.close", it works just as before.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/2/main.rs">listings/actions/2/main.rs</a>

```rust
{{#rustdoc_include ../listings/actions/2/main.rs:accel}}
```

Also, if we had multiple instances of the same windows we would expect that only the currently focused window will be closed when activating "win.close".
And indeed, the "win.close" will be dispatched to the currently focused window.
However, that also means that we actually define one action per window instance.
If we want to have a single globally accessible action instead, we call `add_action_entries` on our application instead.

> Adding "win.close" was useful as a simple example.
> However, in the future we will use the pre-defined ["window.close"](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Window.html#actions) action which does exactly the same thing.

## Parameter and State

An action, like most functions, can take a parameter.
However, unlike most functions it can also be stateful.
Let's see how this works.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/3/main.rs">listings/actions/3/main.rs</a>

```rust
{{#rustdoc_include ../listings/actions/3/main.rs:build_ui}}
```

Here we created a "win.count" action that increases its state by the given parameter every time it is activated.
It also takes care of updating the `label` with the current state.
The button activates the action with each click while passing "1" as parameter.
This is how our app works:

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/actions_counter.webm" type="video/webm">
  <p>A video which shows that pressing on one button also changes the label below</p>
 </video>
</div>


## Actionable

Connecting actions to the "clicked" signal of buttons is a typical use case, which is why all buttons implement the [`Actionable`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Actionable.html) interface.
This way, the action can be specified by setting the "action-name" property.
If the action accepts a parameter, it can be set via the "action-target" property.
With [`ButtonBuilder`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/builders/struct.ButtonBuilder.html), we can set everything up by calling its methods.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/4/main.rs">listings/actions/4/main.rs</a>

```rust
{{#rustdoc_include ../listings/actions/4/main.rs:button_builder}}
```

Actionable widgets are also easily accessible through the interface builder.
As usual, we build up the window via a composite template.
Within the template we can then set the "action-name" and "action-target" properties.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/5/resources/window.ui">listings/actions/5/resources/window.ui</a>

```xml
{{#rustdoc_include ../listings/actions/5/resources/window.ui}}
```

We will connect the actions and add them to the window in the `Window::setup_actions` method.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/5/window/mod.rs">listings/actions/5/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/5/window/mod.rs:impl_window}}
```

Finally, `setup_actions` will be called within `constructed`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/5/window/imp.rs">listings/actions/5/window/imp.rs</a>

```rust
{{#rustdoc_include ../listings/actions/5/window/imp.rs:object_impl}}
```

This app behaves the same as our previous example, but it will make it simpler for us to add a menu in the following section.


## Menus

If you want to create a [menu](https://developer.gnome.org/hig/patterns/controls/menus.html) you have to use actions and you will want to use the interface builder.
Typically, a menu entry has an action fitting one of these three descriptions:
- no parameter and no state, or
- no parameter and boolean state, or
- string parameter and string state.

Let's modify our small app to demonstrate these cases.
First we extend `setup_actions`.
For the action without parameter or state, we can use the pre-defined "window.close" action.
Therefore we don't have to add anything here.

With the action "button-frame", we manipulate the "has-frame" property of `button`.
Here, the convention is that actions with no parameter and boolean state should behave like toggle actions.
This means that the caller can expect the boolean state to toggle after activating the action. Luckily for us, that is the default behavior for [`gio::PropertyAction`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.PropertyAction.html) with a boolean property.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/6/window/mod.rs">listings/actions/6/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/6/window/mod.rs:action_button_frame}}
```

> A `PropertyAction` is useful when you need an action that manipulates the property of a GObject.
> The property then acts as the state of the action.
> As mentioned above, if the property is a boolean the action has no parameter and toggles the property on activation.
> In all other cases, the action has a parameter of the same type as the property.
> When activating the action, the property gets set to the same value as the parameter of the action.


Finally, we add "win.orientation", an action with string parameter and string state.
This action can be used to change the orientation of `gtk_box`.
Here the convention is that the state should be set to the given parameter.
We don't need the action state to implement orientation switching, however it is useful for making the menu display the current orientation.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/6/window/mod.rs">listings/actions/6/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/6/window/mod.rs:action_orientation}}
```

Even though [`gio::Menu`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Menu.html) can also be created with the bindings, the most convenient way is to use the interface builder for that.
We do that by adding the menu in front of the template.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/6/resources/window.ui">listings/actions/6/resources/window.ui</a>

```diff
 <?xml version="1.0" encoding="UTF-8"?>
 <interface>
+  <menu id="main-menu">
+    <item>
+      <attribute name="label" translatable="yes">_Close window</attribute>
+      <attribute name="action">window.close</attribute>
+    </item>
+    <item>
+      <attribute name="label" translatable="yes">_Toggle button frame</attribute>
+      <attribute name="action">win.button-frame</attribute>
+    </item>
+    <section>
+      <attribute name="label" translatable="yes">Orientation</attribute>
+      <item>
+        <attribute name="label" translatable="yes">_Horizontal</attribute>
+        <attribute name="action">win.orientation</attribute>
+        <attribute name="target">Horizontal</attribute>
+      </item>
+      <item>
+        <attribute name="label" translatable="yes">_Vertical</attribute>
+        <attribute name="action">win.orientation</attribute>
+        <attribute name="target">Vertical</attribute>
+      </item>
+    </section>
+  </menu>
   <template class="MyGtkAppWindow" parent="GtkApplicationWindow">
     <property name="title">My GTK App</property>
+    <property name="width-request">360</property>
+    <child type="titlebar">
+      <object class="GtkHeaderBar">
+        <child type ="end">
+          <object class="GtkMenuButton">
+            <property name="icon-name">open-menu-symbolic</property>
+            <property name="menu-model">main-menu</property>
+          </object>
+        </child>
+      </object>
+    </child>
     <child>
       <object class="GtkBox" id="gtk_box">
         <property name="orientation">vertical</property>
```

Since we connect the menu to the [`gtk::MenuButton`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.MenuButton.html) via the [menu-model](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.MenuButton.html#menu-model) property, the `Menu` is expected to be a [`gtk::PopoverMenu`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.PopoverMenu.html).
The [documentation](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.PopoverMenu.html) for `PopoverMenu` also explains its `xml` syntax for the interface builder.

Also note how we specified the target:

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


<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/actions_menu.webm" type="video/webm">
  <p>A video which now also shows the menu</p>
 </video>
</div>

>We changed the icon of the `MenuButton` by setting its property "icon-name" to "open-menu-symbolic".
>You can find more icons with the [Icon Library](https://flathub.org/apps/org.gnome.design.IconLibrary).
>They can be embedded with [`gio::Resource`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Resource.html) and then be referenced within the composite templates (or other places).

## Settings

The menu entries nicely display the state of our stateful actions, but after the app is closed all changes to that state are lost.
As usual, we solve this problem with [`gio::Settings`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Settings.html).
First we create a schema with settings corresponding to the stateful actions we created before.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/org.gtk_rs.Actions7.gschema.xml">listings/actions/7/org.gtk_rs.Actions7.gschema.xml</a>

```xml
{{#rustdoc_include ../listings/actions/7/org.gtk_rs.Actions7.gschema.xml}}
```

Again, we install the schema as described in the settings [chapter](./settings.html).
Then we add the settings to `imp::Window`.
Since [`gio::Settings`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Settings.html) does not implement `Default`, we wrap it in a [`std::cell::OnceCell`](https://doc.rust-lang.org/std/cell/struct.OnceCell.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/window/imp.rs">listings/actions/7/window/imp.rs</a>

```rust
{{#rustdoc_include ../listings/actions/7/window/imp.rs:imp_struct}}
```

Now we create functions to make it easier to access settings.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/window/mod.rs">listings/actions/7/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/7/window/mod.rs:settings}}
```


Creating stateful actions from setting entries is so common that `Settings` provides a method for that exact purpose.
We create actions with the[ `create_action`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/prelude/trait.SettingsExt.html#tymethod.create_action) method and then add them to the action group of our window.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/window/mod.rs">listings/actions/7/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/7/window/mod.rs:settings_create_actions}}
```

Since actions from `create_action` follow the aforementioned conventions, we can keep further changes to a minimum.
The action "win.button-frame" toggles its state with each activation and the state of the "win.orientation" action follows the given parameter.

We still have to specify what should happen when the actions are activated though.
For the stateful actions, instead of adding callbacks to their "activate" signals we bind the settings to properties we want to manipulate.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/window/mod.rs">listings/actions/7/window/mod.rs</a>

```rust
{{#rustdoc_include ../listings/actions/7/window/mod.rs:bind_settings}}
```

Finally, we make sure that `bind_settings` is called within `constructed`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/actions/7/window/imp.rs">listings/actions/7/window/imp.rs</a>

```rust
{{#rustdoc_include ../listings/actions/7/window/imp.rs:object_impl}}
```

Actions are extremely powerful and we are only scratching the surface here.
If you want to learn more about them, the [GNOME developer documentation](https://developer.gnome.org/documentation/tutorials/actions.html) is a good place to start.
