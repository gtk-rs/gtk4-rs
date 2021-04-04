# GObject Concepts

GTK is an object-oriented framework.
It is written in C, which does not support object-orientation out of the box.
That is why, GTK relies on the GObject library to provide the object system.

We already learned, that `gtk-rs` maps GObject concepts like inheritance and interfaces to Rust traits.
In this chapter we will additionally find out:
- How to manage the memory of GObjects.
- How to create our own GObjects via subclassing.
- How to use properties.
- How to use signals.