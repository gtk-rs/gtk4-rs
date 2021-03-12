# Settings

We now learned multiple ways to handle state.
However, every time we close the application all of it is gone.
One common way to persistently store state is via `GSettings`.

First let us create our usual example with a window containing a button.
This time we will take a [`Switch`](https://gtk-rs.org/gtk4-rs/gtk4/struct.Switch.html) though ([`SwitchBuilder`](https://gtk-rs.org/gtk4-rs/gtk4/struct.SwitchBuilder.html) with the builder pattern).
The only difference to `Button` is that it keeps being pressed after being clicked on.
With another click, it returns to its original state.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/settings_1/src/main.rs:button}}
```

<div style="text-align:center"><img src="img/settings_buttons.png" /></div>
