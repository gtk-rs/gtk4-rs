# Subclassing

GObjects rely heavily on inheritance.
Therefore, it makes sense that if we want to create a custom GObject, this is done via subclassing.
Let us see how this works by replacing the button in our "Hello World!" app with a custom one.

First, we need to create an implementation struct that holds the state and overrides the virtual methods.
It is advised to keep it in a private module, since its state and methods are only meant to be used by the GObject itself.
It therefore corresponds to the private section of objects in languages like Java and C++.

<span class="filename">Filename: listings/gobject_subclassing/1/custom_button/imp.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing/1/custom_button/imp.rs}}
```
The description of the subclassing is in `ObjectSubclass`.
- `NAME` should consist of crate-name, module-path and object-name in order to avoid name collisions. Use [PascalCase](https://wiki.c2.com/?PascalCase) here.
- `Type` refers to the actual GObject that will be created afterwards.
- `ParentType` is the GObject we inherit of.

After that, we would have the option to override the virtual methods of our ancestors.
Since we only want to have a plain button for now, we override nothing.
We still have to add the empty `impl` though.
Next, we describe our custom GObject.

<span class="filename">Filename: listings/gobject_subclassing/1/custom_button/mod.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing/1/custom_button/mod.rs}}

# // Please ignore this line
# // It is only there to make rustdoc happy
# fn main() {}
```

[`glib::wrapper!`](http://gtk-rs.org/docs/glib/macro.glib_wrapper.html) does the most of the work of subclassing for us.
We only have to point to the implementation struct and which ancestor GObjects we extend.
Please note that the list of ancestor GObjects does not mention `glib::Object`.
This is because `glib::Object` is *always* the base class in the object hierarchy and therefore already implied.

After these steps, nothing is stopping us anymore from replacing `gtk::Button` with our `CustomButton`.

<span class="filename">Filename: listings/gobject_subclassing/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing/1/main.rs}}
```
We are able to use `CustomButton` as a drop-in replacement for `gtk::Button`.
This is cool, but also not very tempting to do in a real application.
For the gain of zero benefits, it did involve quite a bit of boilerplate after all.

So let us make it a bit more interesting!
`gtk::Button` does not hold much state, but we can let `CustomButton` hold a number.

<span class="filename">Filename: listings/gobject_subclassing/2/custom_button/imp.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing/2/custom_button/imp.rs}}

# // Please ignore this line
# // It is only there to make rustdoc happy
# fn main() {}
```
We override `constructed` in `ObjectImpl` so that the label of the button initializes with `number`.
We also override `clicked` in `ButtonImpl` so that every click increases `number` and updates the label.

<span class="filename">Filename: listings/gobject_subclassing/2/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing/2/main.rs:activate}}
```

In `build_ui` we stop calling `connect_clicked`, and that was it.
After a rebuild, the app now features our `CustomButton` with the label "0".
Every time we click on the button, the number displayed by the label increases by 1.

<div style="text-align:center"><img src="img/gobject_subclassing.png" /></div>

So, when do we want to inherit from GObject?
- We want to use a certain widget, but with added state and overridden virtual functions.
- We want to pass a Rust object to a function, but the function expects a GObject.
- We want to add properties or signals to an object.
