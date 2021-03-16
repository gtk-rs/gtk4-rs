# Properties

Signals are very useful, but when it comes to dealing with GObject state, properties are preferred over raw signals.
With properties you get:
- setters and getters
- emitted "notify" signal whenever it gets set
- ability to bind properties of different GObjects

Typically, a property corresponds to a single member of your GObject struct.
You will like also want to use `Cell` instead of `RefCell` for your property member.
Unlike `RefCell`, `Cell` has no overhead but only allows you to swap the value — not modify it.

Since properties already provide a "notify" signal, we can drop our "number-changed" signal and create a property "number" instead.