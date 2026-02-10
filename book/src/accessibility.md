# Accessibility

GTK strives to be accessible by default.
Most built-in widgets already expose the right information to assistive technologies like screen readers.
However, when you build custom widgets or use widgets in unusual ways, you may need to provide additional accessibility information.

GTK's accessibility is built on the [`Accessible`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.AccessibleExt.html) interface.
Every widget implements this interface, which allows it to communicate with assistive technologies through a tree of accessible objects.

## Accessible Labels and Descriptions

When a widget doesn't have visible text, assistive technologies have no way to describe it to users.
This commonly happens with icon-only buttons.
The solution is to set an accessible label.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/accessibility/1/main.rs">listings/accessibility/1/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/accessibility/1/main.rs:icon_button}}
```

The [`update_property`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.AccessibleExtManual.html#method.update_property) method lets you set accessible properties like `Label` (a short, descriptive name) and `Description` (additional context).
The `Label` is what screen readers announce when the widget receives focus.

For more complex cases, you can also set a `Description` to provide additional context.

```rust,no_run
{{#rustdoc_include ../listings/accessibility/1/main.rs:description}}
```

This is what the icon buttons look like — for sighted users it is typically easier to guess the meaning of the symbol from context. Many icons have no unique name, for example for the right icon how should the screen reader know whether the right icon is described by "settings", "emblem-system" or "gear"?  That's why screen readers need the accessible labels we just set.

<div style="text-align:center"><img src="img/accessibility_labels.png" alt="Window with a search and a settings icon button"/></div>

> When a widget already has visible text (like a regular button with a label), GTK automatically uses that text as the accessible label.
> You only need to set it manually for widgets without visible text.

## Accessible Relationships

Sometimes widgets are related to each other in ways that help users understand the interface.
For example, a label might describe an adjacent entry field.
You can express this relationship using [`accessible::Relation`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/accessible/enum.Relation.html).

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/accessibility/2/main.rs">listings/accessibility/2/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/accessibility/2/main.rs:labelled_by}}
```

When a screen reader focuses the entry, it will announce the label's text, so the user knows what information to enter.

The label and entry are visually next to each other, but the `LabelledBy` relation makes that connection explicit for assistive technologies.

<div style="text-align:center"><img src="img/accessibility_relationship.png" alt="Window with a username label and an entry field"/></div>

## Accessible Roles

Every accessible widget has a role that describes what kind of element it represents.
GTK assigns sensible default roles to built-in widgets: a `Button` has the role `Button`, a `Label` has the role `Label`, and so on.

You can see the available roles in the [`AccessibleRole`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/enum.AccessibleRole.html) enum.
For most widgets, you won't need to change the default role.
However, when creating custom widgets, you should specify a role explicitly.

Let's say you build a custom widget that behaves like a button.
Without setting the role, assistive technologies won't know this is an interactive element.
You should set the accessible role at the class level in `class_init`, so that every instance of your widget carries the correct role.
The widget should also be focusable, so that keyboard users can reach it.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/accessibility/3/main.rs">listings/accessibility/3/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/accessibility/3/main.rs:custom_button}}
```

By setting `AccessibleRole::Button` in `class_init`, screen readers will now announce this as a button.

## Accessible States

Widgets can have states that affect how assistive technologies present them.
The [`accessible::State`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/accessible/enum.State.html) enum defines states like `Checked`, `Disabled`, `Expanded`, and more.

For built-in widgets like `CheckButton` or `Expander`, GTK manages these states automatically.
But when you compose widgets into your own patterns, you need to update the state yourself.

Let's build a collapsible section that properly communicates its expanded state:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/accessibility/4/main.rs">listings/accessibility/4/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/accessibility/4/main.rs:collapsible}}
```

Now screen readers will announce whether the section is expanded or collapsed when users interact with the toggle button.

## Keyboard Navigation

Accessible applications must be fully usable without a mouse.
GTK handles basic keyboard navigation automatically: users can press Tab to move between widgets and Enter or Space to activate buttons.

However, you should ensure that:

1. **All interactive elements can receive focus.** If you create a custom interactive widget from a non-focusable base, set it to be focusable.
   Use [`set_focus_on_click`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/prelude/trait.WidgetExt.html#tymethod.set_focus_on_click) to also let users focus the widget by clicking it.

2. **Focus order is logical.** By default, focus follows the widget hierarchy.

3. **Custom keyboard shortcuts are documented and discoverable.** Consider adding them to your application's shortcuts window.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/accessibility/5/main.rs">listings/accessibility/5/main.rs</a>

```rust,no_run
{{#rustdoc_include ../listings/accessibility/5/main.rs:focusable}}
```

## Testing Accessibility

Testing is essential for ensuring your application works well with assistive technologies.

### Orca Screen Reader

[Orca](https://orca.gnome.org/) is the GNOME screen reader.
You can enable it in Settings → Accessibility → Screen Reader, or toggle it with **Super+Alt+S** (Super is typically the Windows key).
Testing with an actual screen reader is the best way to experience what your users experience.
Try navigating your application using only the keyboard while Orca announces elements.

### Accessibility Checklist

When reviewing your application's accessibility, check that:

- All interactive elements have accessible labels
- Icon-only buttons have descriptive accessible labels
- Custom widgets have appropriate roles
- Dynamic state changes are reflected in accessible states
- The application is fully usable with keyboard only
- Focus is visible and moves in a logical order
- Color is not the only way to convey information

GTK's accessibility support is extensive.
For more details, refer to the [GTK accessibility documentation](https://docs.gtk.org/gtk4/section-accessibility.html).
