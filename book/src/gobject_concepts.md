# GObject Concepts

GTK is an object-oriented framework.
It is written in C, which does not include object-orientation.
That is why, GTK relies on the GObject library to provide the object system.

We already learned, that GObject concepts like inheritance and interfaces are both mapped to Rust traits when we use `gtk-rs`.
In this chapter we will learn:

- How to manage the memory of GObjects.
- How to create our own GObjects via subclassing.
- How to use properties.
- How to use signals.