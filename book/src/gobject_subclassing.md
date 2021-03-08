# Subclassing

GObjects heavily rely on inheritance.
Therefore, it makes sense that if we want to create a custom GObject, this is done via subclassing.
Let's see how subclassing works by replacing the button in our “Hello World!” app with a custom one.

GObjects in `gtk-rs` are made up of two structs.
The first one holds the state and overrides the virtual methods.
It is advised to keep it in a private module,
since it is not supposed to be used directly.

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_1/src/main.rs:impl}}
```
The description of the subclassing is in `ObjectSubclass`
and probably involves more boilerplate than you are used from other languages.
You can choose which `NAME` you want, as long as it is unique within your application.
`Type` refers to the actual GObject we will create afterwards.
`ParentType` is the GObject we inherit of.
After that we would have the option to override the virtual methods of our ancestors.
Since we only want to have a plain button for now, we override nothing.
We still have to add the empty `impl` though.

Now we create the GObject we will actually use in the end.

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_1/src/main.rs:gobject}}
```

`glib::wrapper!` does the most of the work of subclassing for us.
We only have to include the information, where the implementation is (`imp::CustomButton`)
and which ancestor GObjects we extend (`gtk::Button` and `gtk::Widget`).
Please note that we don't have to specify `glib::Object` here.
That is because `glib::Object` is *always* the base class in the object hierarchy.

After we these steps, nothing is stopping us anymore from replacing `gtk::Button` with our `CustomButton`.

```rust ,no_run
{{#rustdoc_include ../listings/gobject_subclassing_1/src/main.rs:call}}
```
We are able to use `CustomButton` as a drop-in replacement for `gtk::Button`.
This is cool, but also not very tempting to do in a real application.
For the gain of zero benefits, it did involve quite a bit of boilerplate after all.

So let's make it a bit more interesting.
A `gtk::Button` contains no state, but we can let `CustomButton` hold a number.
We override `constructed` in `ObjectImpl` so that the label of the button initializes with `number`.
We also override `clicked` in `ButtonImpl` so that every click increases `number` and updates the label.

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_2/src/main.rs:impl}}
```

In `on_activate` we stop calling `connect_clicked`, and that was it.

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_2/src/main.rs:activate}}
```

Now the app features our `CustomButton` with the label “0” (the `Default` for `i32`).
Every time we click on the button, the number displayed by the label increases by 1.

<div style="text-align:center"><img src="images/gobject_subclassing.png" /></div>
