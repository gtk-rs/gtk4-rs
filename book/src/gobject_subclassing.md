# Subclassing

GObjects rely heavily on inheritance.
Therefore, it makes sense that if we want to create a custom GObject, this is done via subclassing.
Let's see how this works by replacing the button in our "Hello World!" app with a custom one.
First, we need to create an implementation struct that holds the state and overrides the virtual methods.

<span class="filename">Filename: listings/gobject_subclassing/1/custom_button/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_subclassing/1/custom_button/imp.rs}}
```
The description of the subclassing is in `ObjectSubclass`.
- `NAME` should consist of crate-name and object-name in order to avoid name collisions. Use [UpperCamelCase](https://en.wikipedia.org/wiki/Camel_case) here.
- `Type` refers to the actual GObject that will be created afterwards.
- `ParentType` is the GObject we inherit of.

After that, we would have the option to override the virtual methods of our ancestors.
Since we only want to have a plain button for now, we override nothing.
We still have to add the empty `impl` though.
Next, we describe the public interface our custom GObject.

<span class="filename">Filename: listings/gobject_subclassing/1/custom_button/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_subclassing/1/custom_button/mod.rs:mod}}
```

[`glib::wrapper!`](http://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/macro.wrapper.html) implements the same traits that our `ParentType` implements.
Theoretically that would mean that we `ParentType` is also the only thing we have to specify here.
Unfortunately, nobody found yet a good way to do that.
Which why, as of today, subclassing of GObjects in Rust requires to mention all ancestors and interfaces apart from `GObject` and `GInitiallyUnowned`.
You could mention only the ones you actively use, however there is no benefit to gain here apart from cutting down the boilerplate a bit.
For `gtk::Button`, we can look up the ancestors and interfaces in the corresponding [doc page](https://docs.gtk.org/gtk4/class.Button.html#hierarchy) of GTK4.

After these steps, nothing is stopping us from replacing `gtk::Button` with our `CustomButton`.

<span class="filename">Filename: listings/gobject_subclassing/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_subclassing/1/main.rs}}
```

> Describing objects with two structs is an implementation detail that leaks here through.
> `imp::CustomButton` handles the state of the GObject and the overridden virtual methods.
> `CustomButton` determines the exposed methods be determining the implemented traits and adding methods itself. 

## Adding Functionality

We are able to use `CustomButton` as a drop-in replacement for `gtk::Button`.
This is cool, but also not very tempting to do in a real application.
For the gain of zero benefits, it did involve quite a bit of boilerplate after all.

So let's make it a bit more interesting!
`gtk::Button` does not hold much state, but we can let `CustomButton` hold a number.

<span class="filename">Filename: listings/gobject_subclassing/2/custom_button/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_subclassing/2/custom_button/imp.rs}}
```
We override `constructed` in `ObjectImpl` so that the label of the button initializes with `number`.
We also override `clicked` in `ButtonImpl` so that every click increases `number` and updates the label.

<span class="filename">Filename: listings/gobject_subclassing/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/gobject_subclassing/2/main.rs:activate}}
```

In `build_ui` we stop calling `connect_clicked`, and that was it.
After a rebuild, the app now features our `CustomButton` with the label "0".
Every time we click on the button, the number displayed by the label increases by 1.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/gobject_subclassing.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

So, when do we want to inherit from GObject?
- We want to use a certain widget, but with added state and overridden virtual functions.
- We want to pass a Rust object to a function, but the function expects a GObject.
- We want to add properties or signals to an object.
