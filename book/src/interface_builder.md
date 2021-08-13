# Interface Builder

Until now, whenever we constructed pre-defined widgets we relied on the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
This way we ended up with the most recent form of our trusty "Hello World!" app.

<span class="filename">Filename: listings/widgets/2/main.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/widgets/2/main.rs}}
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

To instantiate the widgets described by the `xml` files we use [`gtk::Builder`](../docs/gtk4/struct.Builder.html)[^1].
All widgets that can be described that way can be found [here](../docs/gtk4/prelude/trait.BuildableExt.html#implementors-1)

<span class="filename">Filename: listings/interface_builder/1/main.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/1/main.rs:build_ui}}
```

This is a bit disappointing.
Even though we have already described the UI in the markup file, the amount of code is still pretty much the same.
There are still cases where it is valuable to know of the existence of `gtk::Builder`.
We will see for example that [`ShortcutsWindow`](../docs/gtk4/struct.ShortcutsWindow.html) is quite a bit easier to instantiate that way.

However, the reason why we talk about the interface builder at all is the existence of composite templates.
Again, composite templates are described by `xml` files.

<span class="filename">Filename: listings/interface_builder/2/window/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/2/window/window.ui}}
```

Even the content seems to be nearly the same.
Before, we described a pre-existing widget.

```xml
<object class="GtkApplicationWindow" id="window">
```

Now, we create a custom widget and let it inherit from a pre-existing one.

```xml
<template class="MyGtkAppWindow" parent="GtkApplicationWindow">
```

Within our code we also have to create a custom widget inheriting from `gtk::ApplicationWindow` to make use of our template.


<span class="filename">Filename: listings/interface_builder/2/window/mod.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/2/window/mod.rs}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

In the private struct, we then add the derive macro `gtk::CompositeTemplate`.
We also specify that the template information comes from a file `window.ui` in the same folder.
Additionally, we can hold a reference to any child of which we specified the `id` in the template.
Since we need the button later on, we add it to the struct.

<span class="filename">Filename: listings/interface_builder/2/window/imp.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/2/window/imp.rs:object}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Within the `ObjectSubclass` trait, we make sure that `NAME` corresponds to `class` in the template and `ParentType` corresponds to `parent` in the template.
We also bind and initialize the template in `class_init` and `instance_init`.

<span class="filename">Filename: listings/interface_builder/2/window/imp.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/2/window/imp.rs:subclass}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Finally, we connect the callback to the "clicked" signal of `button` within `constructed`.
The button is easily available thanks to the stored reference in `self`.

<span class="filename">Filename: listings/interface_builder/2/window/imp.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/2/window/imp.rs:object_impl}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

With composite templates, `build_ui` actually becomes more concise.

<span class="filename">Filename: listings/interface_builder/2/main.rs</span>
```rust,no_run
{{#rustdoc_include ../listings/interface_builder/2/main.rs:build_ui}}
```
Also with regard to capabilities, we get the best of both worlds.

Thanks to custom widgets we can
- keep state and part of it as properties,
- add signals as well as
- override behavior.

Thanks to composite templates we can
- describe complex user interfaces concisely, and
- easily access widgets within the template. 

In the following chapter, we will see how composite templates help us to create slightly bigger apps such as a To-Do app.

-------------------------------------------------

[^1]: Puh, yet another builder? Let us summarize what we have so far:
- [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder), an IDE used to create GNOME apps, 
- [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html), a design pattern used to create objects with many optional parameters and
- [`gtk::Builder`](../docs/gtk4/struct.Builder.html), the interface builder which creates widgets from `xml` files.

That was it with the builders.
Promised!
