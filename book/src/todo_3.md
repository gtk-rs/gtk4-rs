# Let To-Do App Follow GNOME's HIG

Within this chapter we will adapt our To-Do app so that it follow GNOME's [HIG](https://developer.gnome.org/hig/).
Let's start by installing libadwaita and adding the libadwaita crate to our dependencies as explained in the former [chapter](libadwaita.html).

The most simple change we can make is replacing [`gtk::Application`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Application.html) with [`adw::Application`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Application.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/main.rs">listings/todo/5/main.rs</a>

```rust,no_run,noplayground
{{#rustdoc_include ../listings/todo/5/main.rs:main}}
```

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/5/window/mod.rs">listings/todo/5/window/mod.rs</a>

```rust,no_run,noplayground
{{#rustdoc_include ../listings/todo/5/window/mod.rs:new}}
```

`adw::Application` calls [`init`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/functions/fn.init.html) internally and makes sure that translations, types, stylesheets, and icons are set up properly for Libadwaita. 
It also loads stylesheets automatically from resources as long as they are named [accordingly](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.Application.html#automatic-resources).

Looking at our To-Do app we can see that the looks of its widgets changed.
This is because the Default stylesheet provided by GTK has been replaced with the Adwaita stylesheet provided by Libadwaita.

# TODO: Add comparison

If the setting is set to dark, then dark styles are loaded.

# TODO: Show transition

## Start using Libadwaita widgets

We start by replacing all occurrences of `gtk::prelude` and `gtk::subclass::prelude` with [`adw::prelude`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/prelude/index.html) and [`adw::subclass::prelude`](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/subclass/prelude/index.html).
The `adw` preludes re-export the `gtk` ones plus add a couple of Libadwaita specific traits.

In the remainder of this chapter we are going to follow two patterns of GNOME's HIG.
The first one concerns [header bars](https://developer.gnome.org/hig/patterns/containers/header-bars.html).



Now we are going to replace a couple of GTK elements with the corresponding Libadwaita elements.
You can find the relevant subset of the diff below.
To see the complete file just click on the link after "Filename:".

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/6/resources/window.ui">listings/todo/6/resources/window.ui</a>


```diff
-  <template class="TodoWindow" parent="GtkApplicationWindow">
+  <template class="TodoWindow" parent="AdwApplicationWindow">
     <property name="width-request">360</property>
     <property name="title" translatable="yes">To-Do</property>
+    <property name="content">
       <object class="GtkBox">
         <property name="orientation">vertical</property>
+        <property name="hexpand">True</property>
         <child>
+          <object class="AdwHeaderBar">
+            <child type="end">
+              <object class="GtkMenuButton">
+                <property name="icon-name">view-more-symbolic</property>
+                <property name="menu-model">main-menu</property>
+              </object>
+            </child>
           </object>
         </child>
         <child>
           <object class="GtkScrolledWindow">
             <property name="hscrollbar-policy">never</property>
             <property name="min-content-height">360</property>
-            <child>
-              <object class="GtkListView" id="tasks_list" />
-            </child>
+            <property name="child">
+              <object class="GtkViewport">
+                <property name="scroll-to-focus">True</property>
+                <property name="child">
+                  <object class="AdwClamp">
+                    <property name="child">
+                      <object class="GtkBox">
+                        <property name="orientation">vertical</property>
+                        <property name="spacing">18</property>
+                        <property name="margin-top">24</property>
+                        <property name="margin-bottom">24</property>
+                        <property name="margin-start">12</property>
+                        <property name="margin-end">12</property>
+                        <child>
+                          <object class="GtkEntry" id="entry">
+                            <property name="placeholder-text" translatable="yes">Enter a Task…</property>
+                            <property name="secondary-icon-name">list-add-symbolic</property>
+                          </object>
+                        </child>
+                        <child>
+                          <object class="GtkListBox" id="tasks_list">
+                            <property name="visible">False</property>
+                            <property name="selection-mode">none</property>
+                            <style>
+                              <class name="boxed-list" />
+                            </style>
+                          </object>
+                        </child>
+                      </object>
+                    </property>
+                  </object>
+                </property>
+              </object>
+            </property>
           </object>
         </child>
       </object>
+    </property>
   </template>
 </interface>
```
