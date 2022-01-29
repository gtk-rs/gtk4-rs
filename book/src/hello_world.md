# Hello World!

Now that we've got a working installation, let's get right into it!

At the very least, we need to create an `Application` instance with an [application id](https://developer.gnome.org/documentation/tutorials/application-id.html).
For that we use the [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) which many `gtk-rs` objects support.

<span class="filename">Filename: listings/hello_world/1/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/1/main.rs}}
```

It builds fine, but nothing appears on our screen.
GTK warns us that something should be called in its `activate` step.
So let's create a window there.

<span class="filename">Filename: listings/hello_world/2/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/2/main.rs}}
```
That is better!

<div style="text-align:center"><img src="img/hello_world_empty.png" /></div>

Normally we expect to be able to interact with the user interface.
Also, the name of the chapter suggests that the phrase "Hello World!" will be involved.
Note to bring `gtk::Button` into scope.

<span class="filename">Filename: listings/hello_world/3/main.rs</span>

```rust ,no_run,noplayground
{{#rustdoc_include ../listings/hello_world/3/main.rs:build_ui}}
```
There is now a button and if we click on it, its label becomes "Hello World!".
<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/hello_world_button.webm" type="video/webm">
Your browser does not support the video tag.
 </video>
</div>

Wasn't that hard to create our first `gtk-rs` app, right?
Let's now get a better understanding of what we did here.
