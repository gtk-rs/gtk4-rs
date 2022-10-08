# Composite Templates

Until now, whenever we constructed pre-defined widgets we relied on the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html).
As a reminder, that is how we used it to build our trusty "Hello World!" app.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/hello_world/3/main.rs">listings/hello_world/3/main.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/3/main.rs:all}}
```

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/hello_world_button.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

Creating widgets directly from code is fine, but it makes it harder to separate the logic from the user interface.
This is why most toolkits allow to describe the user interface with a markup language and GTK is no exception here.
For example the following `xml` file describes the window widget of the "Hello World!" app.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/resources/window.ui">listings/composite_templates/1/resources/window.ui</a>
```xml
{{#rustdoc_include ../listings/composite_templates/1/resources/window.ui}}
```

The most outer tag always has to be the `<interface>`.
Then you start listing the elements you want to describe.
In order to define a composite template, we specify the name `MyGtkAppWindow` of the custom widget we want to create and the parent [`gtk::ApplicationWindow`](../docs/gtk4/struct.ApplicationWindow.html) it derives of.
These `xml` files are independent of the programming language, which is why the classes have the original names.
Luckily, they all convert like this: `gtk::ApplicationWindow` → `GtkApplicationWindow`.
Then we can specify properties which are listed [here](https://docs.gtk.org/gtk4/class.ApplicationWindow.html) for `ApplicationWindow`.
Since `ApplicationWindow` can contain other widgets we use the `<child>` tag to add a `gtk::Button`.
We want to be able to refer to the button later on so we also set its `id`.

## Resources

In order to embed the template file into our application we take advantage of [`gio::Resource`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/struct.Resource.html).
The files to embed are again described by an `xml` file.
For our template file we also add the `compressed` and `preprocess` attribute in order to reduce the final size of the resources.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/resources/resources.gresource.xml">listings/composite_templates/1/resources/resources.gresource.xml</a>
```xml
{{#rustdoc_include ../listings/composite_templates/1/resources/resources.gresource.xml}}
```

Now we have to compile the resources and link it to our application.
One way to do this is to execute [`gio::compile_resources`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/fn.compile_resources.html) within a cargo [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

First, we have to add `gtk4` as build dependency in `Cargo.toml` by executing:

```
cargo add gtk4 --rename gtk --build
```

Then, we create a `build.rs` at the root of our package with the following content.
This will compile the resources whenever we trigger a build with cargo and then statically link our executable to them.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/build.rs">listings/build.rs</a>
```rust ,no_run,noplayground
use gtk::gio;

fn main() {
    gio::compile_resources(
        "composite_templates/1/resources",
        "composite_templates/1/resources/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
}
```

Finally, we register and include the resources by calling the macro [`gio::resources_register_include!`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/macro.resources_register_include.html).
In your own apps take care to register the resources before creating the `gtk::Application`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/main.rs">listings/composite_templates/1/main.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/1/main.rs}}
```

Within our code we create a custom widget inheriting from `gtk::ApplicationWindow` to make use of our template.
Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/window/mod.rs">listings/composite_templates/1/window/mod.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/1/window/mod.rs}}
```

In the implementation struct, we then add the derive macro [`gtk::CompositeTemplate`](../docs/gtk4_macros/derive.CompositeTemplate.html).
We also specify that the template information comes from a resource of prefix `/org/gtk-rs/example` containing a file `window.ui`.

One very convenient feature of templates is the template child.
You use it by adding a struct member with the same name as one `id` attribute in the template.
[`TemplateChild`](../docs/gtk4/subclass/widget/struct.TemplateChild.html) then stores a reference to the widget for later use.
This will be useful later, when we want to add a callback to our button.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/window/imp.rs">listings/composite_templates/1/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/1/window/imp.rs:object}}
```

Within the `ObjectSubclass` trait, we make sure that `NAME` corresponds to `class` in the template and `ParentType` corresponds to `parent` in the template.
We also bind and initialize the template in `class_init` and `instance_init`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/window/imp.rs">listings/composite_templates/1/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/1/window/imp.rs:subclass}}
```

Finally, we connect the callback to the "clicked" signal of `button` within `constructed`.
The button is easily available thanks to the stored reference in `self`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/1/window/imp.rs">listings/composite_templates/1/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/1/window/imp.rs:object_impl}}
```

## Custom Widgets

We can also instantiate custom widgets within a template file.
First we define `CustomButton` that inherits from `gtk::Button`.
As usual, we define the implementation struct within `imp.rs`.
Note the `NAME` we define here, we will need it later to refer to it in the template.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/2/custom_button/imp.rs">listings/composite_templates/2/custom_button/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/2/custom_button/imp.rs:imp}}
```

We also define the public struct in `mod.rs`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/2/custom_button/mod.rs">listings/composite_templates/2/custom_button/mod.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/2/custom_button/mod.rs:mod}}
```

Since we want to refer to a `CustomButton` now we also have to change the type of the template child to it.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/2/window/imp.rs">listings/composite_templates/2/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/2/window/imp.rs:object}}
```

Finally, we can replace `GtkButton` with `MyGtkAppCustomButton` within our composite template.
Since the custom button is a direct subclass of `gtk::Button` without any modifications, the behavior of our app stays the same.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/2/resources/window.ui">listings/composite_templates/2/resources/window.ui</a>
```xml
{{#rustdoc_include ../listings/composite_templates/2/resources/window.ui}}
```

## Template Callbacks

We can even specify the handlers of signals within composite templates.
This can be done with a `<signal>` tag containing the name of the signal and the handler in our Rust code.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/3/resources/window.ui">listings/composite_templates/3/resources/window.ui</a>
```xml
{{#rustdoc_include ../listings/composite_templates/3/resources/window.ui}}
```

Then we define the `handle_button_clicked` with the [`template_callbacks`](../docs/gtk4_macros/attr.template_callbacks.html) macro applied to it.
We can determine the function signature by having a look at the `connect_*` method of the signal we want to handle.
In our case that would be [`connect_clicked`](../docs/gtk4/prelude/trait.ButtonExt.html#tymethod.connect_clicked).
It takes a function of type `Fn(&Self)`.
`Self` refers to our button.
This means that `handle_button_clicked` has a single parameter of type `&CustomButton`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/3/window/imp.rs">listings/composite_templates/3/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/3/window/imp.rs:template_callbacks}}
```

Then we have to bind the template callbacks with [`bind_template_callbacks`](../docs/gtk4/subclass/widget/trait.CompositeTemplateCallbacksClass.html#tymethod.bind_template_callbacks).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/3/window/imp.rs">listings/composite_templates/3/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/3/window/imp.rs:subclass}}
```

We can also access the state of our widget.
Let's say we want to manipulate a `number` stored in `imp::Window`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/4/window/imp.rs">listings/composite_templates/4/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/4/window/imp.rs:object}}
```

In order to access the widget's state we have to add `swapped="true"` to the `signal` tag.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/4/resources/window.ui">listings/composite_templates/4/resources/window.ui</a>
```xml
{{#rustdoc_include ../listings/composite_templates/4/resources/window.ui}}
```

Now we can add `&self` as first parameter to `handle_button_clicked`.
This lets us access the state of the window and therefore manipulate `number`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/4/window/imp.rs">listings/composite_templates/4/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/4/window/imp.rs:template_callbacks}}
```

## Registering Types

Now that we use template callbacks we don't access the template child anymore.
Let's remove it. 

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/5/window/imp.rs">listings/composite_templates/5/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/5/window/imp.rs:object}}
```

However, when we now run it GTK doesn't see `MyGtkAppCustomButton` as valid object type anymore.
So what happened here?

```
Gtk-CRITICAL **: Error building template class 'MyGtkAppWindow' for an instance of
                 type 'MyGtkAppWindow': Invalid object type 'MyGtkAppCustomButton'
```

Turns out adding a template child not only gives a convenient reference to a widget within the template.
It also ensures that the widget type is registered.
Luckily we can also do that by ourselves.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/composite_templates/6/window/imp.rs">listings/composite_templates/6/window/imp.rs</a>
```rust ,no_run,noplayground
{{#rustdoc_include ../listings/composite_templates/6/window/imp.rs:subclass}}
```

We call the `ensure_type` method within `class_init` and voilà: our app works again.


## Conclusion

Thanks to custom widgets we can
- keep state and part of it as properties,
- add signals as well as
- override behavior.

Thanks to composite templates we can
- describe complex user interfaces concisely, 
- easily access widgets within the template as well as
- specify handler functions for signals. 

The API involved here is extensive so especially at the beginning you will want to check out the documentation.
The basic syntax of the `ui` files is explained within [`Builder`](../docs/gtk4/struct.Builder.html#gtkbuilder-ui-definitions), syntax specific to widgets within [`Widget`](../docs/gtk4/struct.Widget.html#gtkwidget-as-gtkbuildable).
If a certain widget accepts additional element, then they are typically explained in the docs of the widget.

In the following chapter, we will see how composite templates help us to create slightly bigger apps such as a To-Do app.
