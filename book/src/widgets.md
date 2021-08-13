# Widgets

Widgets are the components that make up a GTK application.
GTK offers many-preexisting ones and if those do not fit, you can even create custom ones.
There are display widgets, buttons, containers and windows.
One kind of widget might be able to contain other widgets, it might present information and it might react to interaction.

The [Widget Gallery](https://docs.gtk.org/gtk4/visual_index.html) is useful to find out which widget fits your needs.
Let us say we want to add a button to our app.
We have quite a bit of choice here, but let us take the simplest one — a `Button`.

<div style="text-align:center"><img src="img/widgets_button.png" /></div>

GTK is an object-oriented framework, so all widgets are part of an inheritance tree with `GObject` at the top.
The inheritance tree of a `Button` looks like this:

```console
GObject
╰── Widget
    ╰── Button
```

The [GTK documentation](https://docs.gtk.org/gtk4/class.Button.html) also tells us that `Button` implements the interfaces `GtkAccessible`, `GtkActionable`, `GtkBuildable`, `GtkConstraintTarget`.

Now let us compare that with the corresponding `Button` struct in `gtk-rs`.
The [gtk-rs documentation](../docs/gtk4/struct.Button.html#implements) tells us which methods and traits it implements.
We find that these traits either have a corresponding base class or interface in the GTK docs.
In the "Hello World" app we wanted to react to a button click.
This behavior is specific to a button, so we expect to find a suitable method in the `ButtonExt` trait.
And indeed, `ButtonExt` includes the method [`connect_clicked`](../docs/gtk4/trait.ButtonExt.html#tymethod.connect_clicked).

<span class="filename">Filename: listings/widgets/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/widgets/1/main.rs:button}}
```

Please note that Rust requires bringing traits into scope, before using one of its methods.
In our example we did that by adding the following line:

<span class="filename">Filename: listings/widgets/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/widgets/1/main.rs:prelude}}
```
With it, we import all necessary traits for dealing with widgets.
You probably want to bring the prelude into scope in most of your source files.

This is also a good moment to mention that most `gtk-rs` objects support the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
This is especially nice for creating widgets where multiple characteristics are already known during their creation.
By utilizing the `builder` method, the construction of our objects becomes quite a bit neater.

<span class="filename">Filename: listings/widgets/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/widgets/2/main.rs}}
```
