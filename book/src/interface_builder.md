# Interface Builder

Until now, whenever we constructed pre-defined widgets we relied on the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
This way we ended up with the most recent form of our trusty "Hello World!" example.

<span class="filename">Filename: listings/widgets/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/widgets/2/main.rs}}
```

Creating widgets directly from code is perfectly fine, toolkits such as Flutter even rely exclusively on this.
However, with most toolkits you can describe your user interface with a markup language and GTK is no exception here.
Describing the window of our "Hello World!" app would look like this:

<span class="filename">Filename: listings/interface_builder/1/window.ui</span>

```xml
{{#rustdoc_include ../listings/interface_builder/1/window.ui}}
```

Compare it with the code above, and you should quickly get the gist of it.
When we build the To-Do app in the following chapters we will have a bigger and more nested widget structure.
There, it will become obvious how much easier it is to keep an overview that way.

To instantiate the widgets described by the `.ui` files we use [`gtk::Builder`](../docs/gtk4/struct.Builder.html)[^1].

<span class="filename">Filename: listings/interface_builder/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/interface_builder/1/main.rs}}
```

That is a bit disappointing.
Even though we have already described the UI in the markup file, the amount of code has not reduced one bit.
There are still cases where it is valuable to know of the existence of `gtk::Builder`.
We will see for example that [`ShortcutsWindow`](../docs/gtk4/struct.ShortcutsWindow.html) is quite a bit easier to instantiate that way.

The real reason why we talk about the interface builder at all is the existence of composite templates.


[^1]: Puh, yet another builder? Let us summarize what we have so far:
- [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder), an IDE used to create GNOME apps, 
- [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html), a design pattern used to create objects with many optional parameters and
- [`gtk::Builder`](../docs/gtk4/struct.Builder.html), the interface builder which creates widgets from `xml` files.

That was it with the builders.
Promised!
