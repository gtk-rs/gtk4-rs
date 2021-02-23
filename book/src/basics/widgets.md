# Widgets

Every GTK app is composed of different kinds of widgets.
For most common operations a pre-existing widget is ready to be used. 

There are display widgets, buttons, containers and windows.
One kind of widget might be able to contain other widgets, it might present information and it might react to interaction.

The [Widget Gallery](https://developer.gnome.org/gtk4/stable/ch08.html) is useful to find out which widget fits your needs.
Let's say we want a button.
We have quite a bit of choice here, but let's take most simple one.
Its name can be found out by clicking on the image ⇒ it is `GTKButton`.

<div style="text-align:center"><img src="https://developer.gnome.org/gtk4/stable/button.png" /></div>

Now let's check out the [gtk-rs docs](https://gtk-rs.org/gtk4-rs/gtk4/index.html) (**TO-DO: Use the new doc link as soon as it is released**).
Rust offers namespaces, so we can drop the “GTK” of `GTKButton`.
A search for `Button` then tells us which methods and traits it [implements](https://gtk-rs.org/gtk4-rs/gtk4/struct.Button.html#implements).
Assuming that we want to connect a callback as we did in the `Hello World` app, we are mostly interested in its “button-behavior”.
And indeed, the trait `ButtonExt` includes the method [`connect_clicked`](https://gtk-rs.org/gtk4-rs/gtk4/trait.ButtonExt.html#tymethod.connect_clicked) which is all we need to do so.
