# The main event loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and scheduling the responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="img/main_event_loop.png" /></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
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

After we press the button, the GUI is completely frozen for 10 seconds.
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

Normally we want to keep track of the work in the thread.
In our case, we don't want the user to spawn additional threads while an existing one is still running.
In order to achieve that we can create a channel.
The main loop allows us to send a message from multiple places to a single receiver at the main thread.
We want to send a `bool` to inform, whether we want the button to react to clicks, or not.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
{{#rustdoc_include ../listings/main_event_loop_3/src/main.rs:callback}}
```

