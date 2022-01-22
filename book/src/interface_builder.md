# Interface Builder

## GTK Builder

Until now, whenever we constructed pre-defined widgets we relied on the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html).
As a reminder, that is how we used it in our trusty "Hello World!" app.

<span class="filename">Filename: listings/hello_world/3/main.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/3/main.rs:all}}
```

Creating widgets directly from code is perfectly fine.
However, with most toolkits you can describe your user interface with a markup language and GTK is no exception here.
For example the following `xml` snippet describes the window widget of the "Hello World!" app. 

<span class="filename">Filename: listings/interface_builder/1/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/1/window.ui}}
```

The most outer tag always has to be the `<interface>`.
Then you start listing the elements you want to describe.
In our case, we want to have a `gtk::ApplicationWindow`.
These `xml` files are independent of the programming language, which is why the classes have the original names.
Luckily, they all convert like this: `gtk::ApplicationWindow` → `GtkApplicationWindow`.
We want to access the window later on, so we also give it an `id`.
Then we can specify properties which are specified [here](https://docs.gtk.org/gtk4/class.ApplicationWindow.html) for `ApplicationWindow`.
Since `ApplicationWindow` can contain other widgets we use the `<child>` tag to add a `Button`.

To instantiate the widgets described by the `xml` files we use [`gtk::Builder`](../docs/gtk4/struct.Builder.html).
All widgets that can be described that way can be found [here](../docs/gtk4/prelude/trait.BuildableExt.html#implementors-1)

> Puh, yet another builder? Let's summarize what we have so far:
> - [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder), an IDE used to create GNOME apps, 
> - [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html), a design pattern used to create objects with many optional parameters and
> - [`gtk::Builder`](../docs/gtk4/struct.Builder.html), the interface builder which creates widgets from `xml` files.
>
> That was it with the builders.
> Promised!

This is how it then looks in practice:

<span class="filename">Filename: listings/interface_builder/1/main.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/1/main.rs:build_ui}}
```

This is a bit disappointing.
Even though we've already described the UI in the markup file, the amount of code is still pretty much the same.
There are still cases where it is valuable to know of the existence of `gtk::Builder`.
We will see for example that [`ShortcutsWindow`](../docs/gtk4/struct.ShortcutsWindow.html) is quite a bit easier to instantiate that way.

At least we did not lose any flexibility by using `gtk::Builder`.
It is for example still possible to refer to custom widgets such as this bare-bones `CustomButton`.

<span class="filename">Filename: listings/interface_builder/2/custom_button/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/2/custom_button/imp.rs:imp}}
```

Within the `xml` file we reference the widget with the `NAME` we gave it in `imp.rs`.

<span class="filename">Filename: listings/interface_builder/3/window/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/2/window.ui}}
```

We also have to make sure to register the custom widget before it is used by the interface builder.

<span class="filename">Filename: listings/interface_builder/2/main.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/2/main.rs:main}}
```

## Composite Templates

The actual reason why we devote a whole chapter to the interface builder is the existence of composite templates.
Again, composite templates are described by `xml` files.

<span class="filename">Filename: listings/interface_builder/3/window/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/3/window/window.ui}}
```

At first glance, the content seems to be nearly the same.
Before, we described a pre-existing widget.

```xml
<object class="GtkApplicationWindow" id="window">
```

Now, we create a custom widget and let it inherit from a pre-existing one.

```xml
<template class="MyGtkAppWindow" parent="GtkApplicationWindow">
```

Within our code we create a custom widget inheriting from `gtk::ApplicationWindow` to make use of our template.

<span class="filename">Filename: listings/interface_builder/3/window/mod.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/3/window/mod.rs}}
```

In the private struct, we then add the derive macro `gtk::CompositeTemplate`.
We also specify that the template information comes from a file `window.ui` in the same folder.

<span class="filename">Filename: listings/interface_builder/3/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/3/window/imp.rs:object}}
```

One very convenient feature of templates is the template child.
You use it by adding a struct member with the same name as one `id` attribute in the template.
Template child then:
- assures that the widget gets registered without doing it manually in `main.rs`, and
- stores a reference to the widget for later use.

We need both for our custom button, so we add it to the struct.

<span class="filename">Filename: listings/interface_builder/3/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/3/window/imp.rs:subclass}}
```

Within the `ObjectSubclass` trait, we make sure that `NAME` corresponds to `class` in the template and `ParentType` corresponds to `parent` in the template.
We also bind and initialize the template in `class_init` and `instance_init`.

<span class="filename">Filename: listings/interface_builder/3/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/3/window/imp.rs:object_impl}}
```

Finally, we connect the callback to the "clicked" signal of `button` within `constructed`.
The button is easily available thanks to the stored reference in `self`.

<span class="filename">Filename: listings/interface_builder/3/main.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/3/main.rs:build_ui}}
```
With composite templates, `main.rs` actually became more concise.
With regard to capabilities, we also get the best of both worlds.

## Conclusion

Thanks to custom widgets we can
- keep state and part of it as properties,
- add signals as well as
- override behavior.

Thanks to composite templates we can
- describe complex user interfaces concisely, and
- easily access widgets within the template. 

The API of the interface builder is extensive so especially at the beginning you will want to check out the documentation.
The basic syntax is explained with [`Builder`](../docs/gtk4/struct.Builder.html#gtkbuilder-ui-definitions), syntax specific to widgets with [`Widget`](../docs/gtk4/struct.Widget.html#gtkwidget-as-gtkbuildable).
If a certain widget accepts additional element, then they are typically explained in the docs of the widget.

In the following chapter, we will see how composite templates help us to create slightly bigger apps such as a To-Do app.
