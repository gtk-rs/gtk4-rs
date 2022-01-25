# CSS

When you want to modify the style of your website, you use [CSS](https://de.wikipedia.org/wiki/Cascading_Style_Sheets).
Similarly, GTK supports its own variant of CSS in order to style your app.

> We will not explain every piece of syntax used in this chapter.
> If you are new to CSS or just need a refresher, have a look at the [MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/CSS/Syntax).

Let's say we have a button and we want to set its font color to magenta.
Every type of widget has a corresponding CSS node.
In the case of `gtk::Button`, this node is called `button`.
Therefore, we create a `style.css` file with the following content:

<span class="filename">Filename: listings/css/1/style.css</span>

```css
{{#rustdoc_include ../listings/css/1/style.css}}
```

Next, we need to load the CSS file in the startup step of the application.
As usual, the widgets are created during the "activate" step.

<span class="filename">Filename: listings/css/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/css/1/main.rs:main}}
```

When we now run the app, we notice that our button *and* the "close" button are magenta.
Probably not what we wanted, but that is what our CSS snippet does.
We did not specify for which button the rule should apply, so it was applied to both of them.

<div style="text-align:center"><img src="img/css_1.png"/></div>

>The `GtkInspector` comes in quite handy (not only) when playing with CSS.
>Make sure that the window of your app is focused and press <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>D</kbd>.
> A window will pop up which lets you browse and even manipulate the state of your app.
> Open the CSS view and override the button color with the following snippet.
>```css
>button {
>  color: blue;
>}
>```
> With the pause button you can toggle whether your CSS code is active or not.

## Style Classes Applied by GTK

[Class selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/Class_selectors) are one way to choose which specific elements a CSS rule applies to.
GTK adds style classes to many of its widgets, often depending on their content.
A [`gtk::Button`](../docs/gtk4/struct.Button.html#css-nodes), for example, will get the `text-button` style class when its content is a label.
That is why we create a new CSS rule which only applies to `button` nodes with the style class `text_button`.


<span class="filename">Filename: listings/css/2/style.css</span>

```css
{{#rustdoc_include ../listings/css/2/style.css}}
```

Now only the font of our button becomes magenta.

<div style="text-align:center"><img src="img/css_2.png"/></div>

## Adding Your Own Style Class

With [`add_css_class`](../docs/gtk4/prelude/trait.WidgetExt.html#tymethod.add_css_class) we can also add our own style classes to widgets.
One use-case for this is when you want a rule to apply to a hand-picked set of widgets.
For example if we have two buttons, but want only one of them to have magenta font.
Relying on one of the style classes which GTK adds will not help since both will get the same ones.
Which is why we add the style class `button-1` to the first one.

<span class="filename">Filename: listings/css/3/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/css/3/main.rs:buttons}}
```

Then, we create a CSS rule that applies to `button` nodes with the style class `button-1`.

<span class="filename">Filename: listings/css/3/style.css</span>

```css
{{#rustdoc_include ../listings/css/3/style.css}}
```

We can see that this way only the first button gets colored magenta.

<div style="text-align:center"><img src="img/css_3.png"/></div>


## Specifying Name of a Widget


If you want that your rule only applies to a single widget, matching with style classes can be fine.
Ideally however, you would give the widget a name and match with that name instead.
This way your intentions are more clear, compared to matching with style classes that can apply to multiple widgets. 

Again, we have two buttons but want to color only one of them magenta.
We set the name of the first one with [`set_widget_name`](../docs/gtk4/prelude/trait.WidgetExt.html#tymethod.set_widget_name).

<span class="filename">Filename: listings/css/4/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/css/4/main.rs:buttons}}
```

Then, create a CSS rule that applies to `button` nodes with the name `button-1`.
The name is specified after the `#` symbol.

<span class="filename">Filename: listings/css/4/style.css</span>

```css
{{#rustdoc_include ../listings/css/4/style.css}}
```

Again, the style rule only applies to the first button.

<div style="text-align:center"><img src="img/css_4.png"/></div>


## CSS Rules Provided by GTK

Certain styles are common enough that GTK provides CSS rules for them.
For example, if you want to indicate that your button leads to a destructive or suggested action you don't have to provide your own CSS rules.
All you have to do is to add "destructive-action" or "suggested-action" style class to your button.
Most widgets will document these rules in their documentation under [CSS nodes](../docs/gtk4/struct.Button.html#css-nodes).

<span class="filename">Filename: listings/css/5/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/css/5/main.rs:buttons}}
```

<div style="text-align:center"><img src="img/css_5.png"/></div>

## Interface Builder

We can also add style classes with the interface builder.
Just add the `<style>` element to your widget.
The `<style>` element is documented together with [`gtk::Widget`](../docs/gtk4/struct.Widget.html#gtkwidget-as-gtkbuildable).
Adding again destructive and suggested buttons, would then look like this:

<span class="filename">Filename: listings/css/6/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/css/6/window/window.ui}}
```


## Pseudo-classes

Sometimes you want your CSS rules to apply under even more precise conditions than style classes allow.
That is where [pseudo-classes](https://developer.mozilla.org/en-US/docs/Web/CSS/Pseudo-classes) come in.
Let's use a single button with name `button-1` to demonstrate this concept.

<span class="filename">Filename: listings/css/7/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/css/7/window/window.ui}}
```

By adding the pseudo-class `hover`, we say that we want this rule to only apply to a `button` node with name `button-1` when hovering over it with the mouse pointer.

<span class="filename">Filename: listings/css/7/style.css</span>

```css
{{#rustdoc_include ../listings/css/7/style.css}}
```

If we now hover over the button, we see that over the span of one second its background turns yellow and its font turns magenta.
After we removed the cursor, the button returns to its original state.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/css_6.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

## Nodes

In the previous examples, a widget always corresponded to a single CSS node.
This is not always the case.
For example, [`gtk::MenuButton`](../docs/gtk4/struct.MenuButton.html) has multiple CSS nodes.
Let's see how that works.

First, we create a single `MenuButton`.


<span class="filename">Filename: listings/css/8/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/css/8/window/window.ui}}
```
You can make a `MenuButton` show an icon or a label.
If you choose to do neither of those, as we currently do, it shows an image displaying an arrow.

An inheritance tree of [CSS nodes](../docs/gtk4/struct.MenuButton.html#css-nodes) displays this situation:

```
menubutton
╰── button.toggle
    ╰── <content>
         ╰── [arrow]
```

We see that the `menubutton` node has children, which themselves have children and attached style classes.
Now we know that we have to add a CSS rule that applies to the `arrow` node, which is a descendant of `menubutton`.

<span class="filename">Filename: listings/css/8/style.css</span>

```css
{{#rustdoc_include ../listings/css/8/style.css}}
```

Indeed, we get a `MenuButton` with a magenta arrow.

<div style="text-align:center"><img src="img/css_8.png"/></div>


## Exported Colors

Now that we know how to use CSS, it is time to update our To-Do app.
Before, the individual tasks were a bit hard to distinguish.
It would be nice if the todo rows would be surrounded by borders.
Let's add that!

The class `TodoRow` inherits from `gtk::Box`, so we could just match for the node `box`.
However, if we create a custom widget we might as well give it its own CSS name.
Keep in mind, that this is not the same as when we gave a specific instance of a widget a name.
When calling [`set_css_name` ](../docs/gtk4/subclass/widget/trait.WidgetClassSubclassExt.html#method.set_css_name), we change the name of the CSS node of a widget.
In our case, the widget `TodoRow` now corresponds to the node `todo-row`.

<span class="filename">Filename: listings/todo_app/3/todo_row/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/3/todo_row/imp.rs:object_subclass}}
```

Now we have to decide which color to use for the borders.
Luckily, the stylesheet that GTK uses provides pre-defined colors for various use-cases.
As of this writing, the exported colors of the default stylesheet can only be found in its [source code](https://gitlab.gnome.org/GNOME/gtk/-/blob/b2c227e9c57839a2a4e24462a71ae0bad9a95264/gtk/theme/Default/_colors-public.scss).

There we find the color `borders`, which should be used for the widget's border color.
We can then access the pre-defined color by adding an `@` in front of its name.

<span class="filename">Filename: listings/todo_app/3/style.css</span>

```css
{{#rustdoc_include ../listings/todo_app/3/style.css}}
```

Now our tasks have borders around them, and we are one step further in finishing our To-Do app.

<div style="text-align:center"><img src="img/todo_app_change.png"/></div>

This was also an excellent opportunity to show how to set the CSS name of custom widget and how to access exported colors.
In the end, we find that GTK provides a style rule to add borders to a node.
This seems nicer, so we will use that instead.
We match the style rule by adding the style class `frame` to our `TodoRow`.

<span class="filename">Filename: listings/todo_app/4/todo_row/todo_row.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/4/todo_row/todo_row.ui}}
```

## Conclusion

There are surely enough ways to define CSS rules.
Let's briefly recap the syntax we learned.
The following rule matches the node `arrow`, which is a descendant of the node `button` with the name `button-1` and the style classes `toggle` and `text-button`.
The rule then actually applies, when we also hover over `arrow`.

```css
button#button-1.toggle.text-button arrow:hover {
  color: magenta;
}
```

When the rule applies, the `color` parameter will be set to magenta.
You can find the full list of supported parameters in GTK's [documentation](https://docs.gtk.org/gtk4/css-properties.html#gtk-css-properties).


In the following chapter we will learn how to make a binary bundle with CSS files, icons and any other resources that our app might need at runtime.
