# Building a Simple To-Do App

After we have learned so many concepts,
now it is finally time to put them into practice.
We are going to build a To-Do app!

For now, we would already be satisfied with a minimal version.
An entry to input new tasks and a list view to display them should suffice.
Something like this:

<div style="text-align:center"><img src="img/todo_app_1.png" /></div>

We can translate this mockup of the window directly into an `xml` file.
This would then look like the following snippet:

<span class="filename">Filename: listings/todo_app/1/window/window.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/1/window/window.ui}}
```

We see the `<template>` tag, so composite templates are involved here.
First, we subclass from `gtk::ApplicationWindow`.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:glib_wrapper}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Then we initialize the composite template for `imp::Window`.
We store references to the entry, the list view as well as the model.
This will come in handy when we later add methods to our window.
After that we add the typical boilerplate for initializing composite templates.

<span class="filename">Filename: listings/todo_app/1/window/imp.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/imp.rs:struct_and_subclass}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

`main.rs` also does not hold any surprises for us.

<span class="filename">Filename: listings/todo_app/1/main.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/main.rs:main}}
```

So far so good.
The main user interface is done, but the entry does not react to input yet.
Also, where would the input go?
We have not even set up the list view yet.
Let us take care of that!

As discussed in the [lists chapter](./lists.html),
we start out by creating a custom GObject.
This object will store the state of the task:
- a boolean describing whether the task is completed or not, and
- a string holding the content.

<span class="filename">Filename: listings/todo_app/1/todo_object/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_object/mod.rs:glib_wrapper_and_new}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Unlike the lists chapter, the state is stored in a struct rather than in individual members of `imp::TodoObject`.

<span class="filename">Filename: listings/todo_app/1/todo_object/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_object/mod.rs:todo_data}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Not only that, we see that it is also wrapped in a [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html).
That does not help us much now, but will be very convenient when we want to save the state in one of the following chapters.
Exposing `completed` and `content` as properties does not become much different that way, so we will not discuss it further.
In you are curious, you can press on the small eye symbol on the top right of the code snippet to check for yourself.

<span class="filename">Filename: listings/todo_app/1/todo_object/imp.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_object/imp.rs:struct_and_subclass}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```


We already determined that the row of a task should then look like this:


<div style="text-align:center"><img src="img/todo_row.png" /></div>

Again, we describe the mockup with a composite template.


<span class="filename">Filename: listings/todo_app/1/todo_row/todo_row.ui</span>

```xml
{{#rustdoc_include ../listings/todo_app/1/todo_row/todo_row.ui}}
```

First we derive `TodoRow` from `gtk::Bin`.

<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:glib_wrapper}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

In `imp::TodoRow`, we hold references to `completed_button` and `content_label`.
We also store a mutable vector of bindings.
Why we need that will become clear as soon as we get to bind the state of `TodoObject` to the corresponding `TodoRow`.


<span class="filename">Filename: listings/todo_app/1/todo_row/imp.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_row/imp.rs:struct_and_subclass}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Let us start with bringing everything together.
We override the `constructed` method of `Window` in order to ensure that everything will be setup immediately after the window is constructed.

<span class="filename">Filename: listings/todo_app/1/window/imp.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/imp.rs:constructed}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Additional methods will be added to `Window` instead of `imp::Window`.
Let us start with the model.
Since accessing the model is a typical use case, we add the convenience method `model` for that.
In `setup_model` we create a new `gio::ListStore` and store a reference to it in `imp::Window` as well as the `gtk::ListView`.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:model}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

For now, we only connect to the "activate" signal of the entry in `setup_callbacks`.
Whenever, we finished up writing our task, we can activate the entry by, for example, pressing the enter key.
When that happens, a new `TodoObject` with the content will be created and appended to the model.
Then the entry will be cleared.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:setup_callbacks}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

Before we move on to the factory, let us first think which behavior we expect here.
`content_label` of `TodoRow` should follow `content` of `TodoObject`.
We already know how to do that with expressions.
We also want `completed_button` of `TodoRow` follow `completed` of `TodoObject`.
However, if we toggle the state of `completed_button`, `completed` should do the same.
Unfortunately, we expressions cannot handle bidirectional relationships.
This means we have to use property bindings and assure ourselves that they get unbound when they are not needed anymore.

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:setup_factory}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:bind}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```


<span class="filename">Filename: listings/todo_app/1/todo_row/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/todo_row/mod.rs:unbind}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```
