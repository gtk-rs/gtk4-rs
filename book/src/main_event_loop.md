# The main event loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and scheduling the responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="images/main_event_loop_states.gif" /></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
You can even spawn [async functions](http://gtk-rs.org/docs/glib/struct.MainContext.html#method.spawn_local) on it.
It does all of that within the same thread.
Quickly iterating between all tasks gives the illusion of parallelism.
That is why you can move the window at the same time as a progress bar is growing.


However, you surely saw GUIs that became unresponsive, at least for a few seconds.
That happens when a single task takes too long.
Let us look at one example.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_1/src/main.rs}}
```

After we press the button, the GUI is completely frozen.
We can't even move the window.
The `sleep` call is an artificial example,
but it is not unusual wanting to run a slightly longer operation in one go.
For that we just need to spawn a new thread and let the operation run there.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_2/src/main.rs:callback}}
```

If you come from another language than Rust, you might be uncomfortable with the thought of spawning new threads before even looking at other options.
Luckily, Rust's safety guarantees allow you to stop worrying about the nasty bugs, concurrency tends to bring.
