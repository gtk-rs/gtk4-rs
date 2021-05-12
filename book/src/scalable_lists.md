# Scalable Lists

In quite a few instances you have more elements than you can display at once.
Just think of the infinite amount of posts in your social media timeline.
Creating a widget for each individual post is impossible, but even for less extreme cases that is typically not what we want.
Ideally, we could specify our data and how it transforms into a widget.
GTK would then reuse the widgets as we scroll through our elements.

<div style="text-align:center" width="20%"><img src="img/scalable_lists_concept.png"/></div>


<span class="filename">Filename: listings/scalable_lists/1/integer_object/mod.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/integer_object/mod.rs:integer_object}}

# // Please ignore this line
# // It is only there to make mdbook happy
# fn main() {}
```

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:model}}
```

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:factory_setup}}
```

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:factory_bind}}
```

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:selection_list}}
```

<span class="filename">Filename: listings/scalable_lists/1/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/scalable_lists/1/main.rs:scrolled_window}}
```
