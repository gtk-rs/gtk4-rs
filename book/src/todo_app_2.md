# Manipulating State of To-Do App


<div style="text-align:center">
 <video autoplay muted loop>
  <source src="img/todo_app_2_animation.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

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

<span class="filename">Filename: listings/todo_app/2/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/imp.rs:struct_default}}
```

<span class="filename">Filename: listings/todo_app/2/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/todo_object/mod.rs:impl}}
```

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:filter}}
```

<span class="filename">Filename: listings/todo_app/2/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/2/window/mod.rs:setup_model}}
```
