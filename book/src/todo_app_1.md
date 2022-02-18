# Building a Simple To-Do App

After we have learned so many concepts, it is finally time to put them into practice.
We are going to build a To-Do app!

For now, we would already be satisfied with a minimal version.
An entry to input new tasks and a list view to display them will suffice.
Something like this:

<div style="text-align:center"><img src="img/todo_app_1_mockup.png" /></div>

## Window

This mockup can be described by the following composite template.

<span class="filename">Filename: listings/todo_app/1/resources/window.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/1/resources/window.ui}}
```


In order to use the composite template, we create a custom widget.
The `parent` is `gtk::ApplicationWindow`, so we inherit from it.
As usual, we have to list all [ancestors](https://docs.gtk.org/gtk4/class.ApplicationWindow.html#ancestors) and [interfaces](https://docs.gtk.org/gtk4/class.ApplicationWindow.html#implements) apart from `GObject` and `GInitiallyUnowned`.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:glib_wrapper}}
```

Then we initialize the composite template for `imp::Window`.
We store references to the entry, the list view as well as the list model.
This will come in handy when we later add methods to our window.
After that, we add the typical boilerplate for initializing composite templates.
We only have to assure that the `class` attribute of the template in `window.ui` matches `NAME`.

<span class="filename">Filename: listings/todo_app/1/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/imp.rs:struct_and_subclass}}
```

`main.rs` also does not hold any surprises for us.

<span class="filename">Filename: listings/todo_app/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/main.rs:main}}
```

Finally, we specify our resources.
Here, they already include `todo_row.ui` which we will handle later in this chapter.

<span class="filename">Filename: listings/todo_app/1/resources/resources.gresource.xml</span>

```xml
{{#rustdoc_include ../listings/todo_app/1/resources/resources.gresource.xml}}
```


## To-Do Object

So far so good.
The main user interface is done, but the entry does not react to input yet.
Also, where would the input go?
We haven't even set up the list model yet.
Let's do that!

<div style="text-align:center"><img src="img/todo_app_1_empty.png" /></div>

As discussed in the [list widgets chapter](./list_widgets.html),
we start out by creating a custom GObject.
This object will store the state of the task consisting of:
- a boolean describing whether the task is completed or not, and
- a string holding the task name.

<span class="filename">Filename: listings/todo_app/1/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_object/mod.rs:glib_wrapper_and_new}}
```

Unlike the lists chapter, the state is stored in a struct rather than in individual members of `imp::TodoObject`.
This will be very convenient when saving the state in one of the following chapters.

<span class="filename">Filename: listings/todo_app/1/todo_object/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_object/mod.rs:todo_data}}
```

Exposing `completed` and `content` as properties does not become much different that way, so we will not discuss it further.
If you are curious, you can press on the small eye symbol on the top right of the code snippet to read the implementation.

<span class="filename">Filename: listings/todo_app/1/todo_object/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_object/imp.rs:struct_and_subclass}}
```

## To-Do Row

Let's move on to the individual tasks.
The row of a task should look like this:


<div style="text-align:center"><img src="img/todo_row.png" /></div>

Again, we describe the mockup with a composite template.


<span class="filename">Filename: listings/todo_app/1/resources/todo_row.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/1/resources/todo_row.ui}}
```

In the code, we [derive](https://docs.gtk.org/gtk4/class.Box.html#hierarchy) `TodoRow` from `gtk:Box`:

<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:glib_wrapper}}
```

In `imp::TodoRow`, we hold references to `completed_button` and `content_label`.
We also store a mutable vector of bindings.
Why we need that will become clear as soon as we get to bind the state of `TodoObject` to the corresponding `TodoRow`.


<span class="filename">Filename: listings/todo_app/1/todo_row/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_row/imp.rs:struct_and_subclass}}
```

Now we can bring everything together.
We override the `imp::Window::constructed` in order to set up window contents at the time of its construction.

<span class="filename">Filename: listings/todo_app/1/window/imp.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/imp.rs:constructed}}
```

Since we need to access the list model quite often, we add the convenience method `Window::model` for that.
In `Window::setup_model` we create a new model.
Then we store a reference to the model in `imp::Window` as well as in `gtk::ListView`.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:model}}
```

In `Window::setup_callbacks` we connect to the "activate" signal of the entry.
This signal is triggered when we press the enter key in the entry.
Then a new `TodoObject` with the content will be created and appended to the model.
Finally, the entry will be cleared.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:setup_callbacks}}
```
The list elements for the `gtk::ListView` are produced by a factory.
Before we move on to the implementation, let's take a step back and think about which behavior we expect here.
`content_label` of `TodoRow` should follow `content` of `TodoObject`.
We also want `completed_button` of `TodoRow` follow `completed` of `TodoObject`.
This could be achieved with expressions similar to what we did in the lists chapter.

However, if we toggle the state of `completed_button` of `TodoRow`, `completed` of `TodoObject` should change too.
Unfortunately, expressions cannot handle bidirectional relationships.
This means we have to use property bindings.
We will need to unbind them manually when they are no longer needed.

We will create empty `TodoRow` objects in the "setup" step in `Window::setup_factory` and deal with binding in the "bind" and "unbind" steps.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:setup_factory}}
```

Binding properties in `TodoRow::bind` works just like in former chapters.
The only difference is that we store the bindings in a vector.
This is necessary because a `TodoRow` will be reused as you scroll through the list.
That means that over time a `TodoRow` will need to bound to a new `TodoObject` and has to be unbound from the old one.
Unbinding will only work if it can access the stored [`glib::Binding`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.Binding.html).

<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:bind}}
```

`TodoRow::unbind` takes care of the cleanup.
It iterates through the vector and unbinds each binding.
In the end, it clears the vector.

<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:unbind}}
```

That was it, we created a basic To-Do app!
We will extend it with additional functionality in the following chapters.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/todo_app_1_animation.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>
