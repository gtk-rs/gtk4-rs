# GObject Concepts

GTK is an object-oriented framework.
It is written in C, which does not support object-orientation out of the box.
That is why GTK relies on the GObject library to provide the object system.

We have already learned that `gtk-rs` maps GObject concepts, like inheritance and interfaces, to Rust traits.
In this chapter we will learn:
- How memory of GObjects is managed
- How to create our own GObjects via subclassing
- How to deal with generic values
- How to use properties
- How to emit and receive signals
