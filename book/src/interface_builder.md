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

This is how it then looks in practice:

<span class="filename">Filename: listings/interface_builder/1/main.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/1/main.rs:build_ui}}
```

This is a bit disappointing.
Even though we have already described the UI in the markup file, the amount of code is still pretty much the same.
There are still cases where it is valuable to know of the existence of `gtk::Builder`.
We will see for example that [`ShortcutsWindow`](../docs/gtk4/struct.ShortcutsWindow.html) is quite a bit easier to instantiate that way.

At least we did not lose any flexibility by using `gtk::Builder`.
It is for example still possible to refer to custom widgets.
Let us test this with `CustomButton` which subclasses from `gtk::Button` without additional modifications. 

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

> If we refer to a custom widget in our template, but do not want to add it as template child then we still have to register it.
> `class_init` is a good place to do so.

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

## Template Callbacks

We can even specify the handlers of signals within composite templates.
This can be done with a `<signal>` tag containing the name of the signal and the handler in our Rust code.

<span class="filename">Filename: listings/interface_builder/4/window/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/4/window/window.ui}}
```

Then we define the `handle_button_clicked` with the [`template_callbacks`](../docs/gtk4_macros/attr.template_callbacks.html) macro applied to it.
We can determine the function signature by having a look at the `connect_*` method of the signal we want to handle.
In our case that would be [`connect_clicked`](../docs/gtk4/prelude/trait.ButtonExt.html#tymethod.connect_clicked).
It takes a function of type `Fn(&Self)`.
`Self` refers to our button.
This means that `handle_button_clicked` has a single parameter of type `&CustomButton`

<span class="filename">Filename: listings/interface_builder/4/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/4/window/imp.rs:template_callbacks}}
```

Then we have to bind the template callbacks with [`bind_template_callbacks`](../docs/gtk4/subclass/widget/trait.CompositeTemplateCallbacks.html#method.bind_template_callbacks).

<span class="filename">Filename: listings/interface_builder/4/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/4/window/imp.rs:subclass}}
```

We can also access the state of our widget.
Let's  say we want to manipulate a `number` stored in `imp::Window`.

<span class="filename">Filename: listings/interface_builder/5/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/5/window/imp.rs:object}}
```

In order to access the widget's state we have to add `swapped="true"` to the `signal` tag.

<span class="filename">Filename: listings/interface_builder/5/window/window.ui</span>
```xml
{{#rustdoc_include ../listings/interface_builder/5/window/window.ui}}
```

Now we can add `&self` as first parameter to `handle_button_clicked`.
This lets us access the state of the window and therefore manipulate `number`.

<span class="filename">Filename: listings/interface_builder/5/window/imp.rs</span>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/interface_builder/5/window/imp.rs:template_callbacks}}
```



## Conclusion

Thanks to custom widgets we can
- keep state and part of it as properties,
- add signals as well as
- override behavior.

Thanks to composite templates we can
- describe complex user interfaces concisely, 
- easily access widgets within the template as well as
- specify handler functions for signals. 

The API of the interface builder is extensive so especially at the beginning you will want to check out the documentation.
The basic syntax is explained with [`Builder`](../docs/gtk4/struct.Builder.html#gtkbuilder-ui-definitions), syntax specific to widgets with [`Widget`](../docs/gtk4/struct.Widget.html#gtkwidget-as-gtkbuildable).
If a certain widget accepts additional element, then they are typically explained in the docs of the widget.

In the following chapter, we will see how composite templates help us to create slightly bigger apps such as a To-Do app.
