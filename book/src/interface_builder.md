# Interface Builder

Until now, whenever we constructed pre-defined widgets we relied on the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
This way we ended up with the most recent form of our trusty "Hello World!" example.

<span class="filename">Filename: listings/widgets/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/widgets/2/main.rs}}
```

Creating widgets directly from code is perfectly fine, toolkits such as [Flutter](https://flutter.dev/) even rely exclusively on this.
However, with most toolkits you can describe your user interface with a markup language and GTK is no exception here.
Describing window our "Hello World!" app would look like this:


<span class="filename">Filename: listings/interface_builder/1/window.ui</span>

```xml
{{#rustdoc_include ../listings/interface_builder/1/window.ui}}
```
