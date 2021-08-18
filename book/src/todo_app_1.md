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
Also where would the input go?
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
We override the `constructed` method of `Window`


<span class="filename">Filename: listings/todo_app/1/window/imp.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/imp.rs:constructed}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```


<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:model}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

<span class="filename">Filename: listings/todo_app/1/window/mod.rs</span>

```rust
{{#rustdoc_include ../listings/todo_app/1/window/mod.rs:setup_callbacks}}
# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

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
