# Implementing the Accessible Text Interface

This example creates a custom text editing widget that implements the Accessible Text Interface. This interface is used for providing screen reader accessibility to custom widgets.

To test this implementation, enable the screen reader (Orca is enabled via Super+Alt+S) and edit / navigate the text.

The widget mostly just delegates its implementation to the parent `TextView`.
