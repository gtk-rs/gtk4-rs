# The Main Event Loop

We now got comfortable using callbacks, but how do they actually work?
All of this happens asynchronously, so there must be something managing the events and scheduling the responses.
Unsurprisingly, this is called the main event loop.

<div style="text-align:center"><img src="img/main_event_loop.png" alt="Diagram showing the main event loop"/></div>

The main loop manages all kinds of events — from mouse clicks and keyboard presses to file events.
It does all of that within the same thread.
Quickly iterating between all tasks gives the illusion of parallelism.
That is why you can move the window at the same time as a progress bar is growing.

However, you surely saw GUIs that became unresponsive, at least for a few seconds.
That happens when a single task takes too long.
The following example uses [`std::thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html) to represent a long-running task.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/1/main.rs">listings/main_event_loop/1/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/1/main.rs}}
```

After we press the button, the GUI is completely frozen for five seconds.
We can't even move the window.
The `sleep` call is an artificial example,
but frequently, we want to run a slightly longer operation in one go.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/main_event_loop_1.webm" type="video/webm">
  <p>A video which shows that after pressing the button, the window can still be moved</p>
 </video>
</div>

## How to Avoid Blocking the Main Loop

In order to avoid blocking the main loop, we can spawn a new task with [`gio::spawn_blocking`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/gio/fn.spawn_blocking.html) and let the operation run on the thread pool.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/2/main.rs">listings/main_event_loop/2/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/2/main.rs:callback}}
```

Now the GUI doesn't freeze when we press the button.
However, nothing stops us from spawning as many tasks as we want at the same time.
This is not necessarily what we want.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/main_event_loop_2.webm" type="video/webm">
  <p>A video which shows that after pressing the button, the window can still be moved</p>
 </video>
</div>

> If you come from another language than Rust, you might be uncomfortable with the thought of running tasks in separate threads before even looking at other options.
> Luckily, Rust's safety guarantees allow you to stop worrying about the nasty bugs that concurrency tends to bring.

## Channels

Typically, we want to keep track of the work in the task.
In our case, we don't want the user to spawn additional tasks while an existing one is still running.
In order to exchange information with the task we can create a channel with the crate [`async-channel`](https://docs.rs/async-channel/latest/async_channel/index.html).
Let's add it by executing the following in the terminal:

```
cargo add async-channel
```

We want to send a `bool` to inform, whether we want the button to react to clicks or not.
Since we send in a separate thread, we can use [`send_blocking`](https://docs.rs/async-channel/latest/async_channel/struct.Sender.html#method.send_blocking).
But what about receiving?
Every time we get a message, we want to set the sensitivity of the button according to the `bool` we've received.
However, we don't want to block the main loop while waiting for a message to receive.
That is the whole point of the exercise after all!

We solve that problem by waiting for messages in an [`async`](https://rust-lang.github.io/async-book/) block.
This `async` block is spawned on the `glib` main loop with [`spawn_future_local`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/fn.spawn_future_local.html)

> See also [`spawn_future`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/fn.spawn_future.html) for spawning async blocks on the main loop from outside the main thread.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/3/main.rs">listings/main_event_loop/3/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/3/main.rs:callback}}
```

As you can see, spawning a task still doesn't freeze our user interface.
However, now we can't spawn multiple tasks at the same time since the button becomes insensitive after the first task has been spawned.
After the task is finished, the button becomes sensitive again.

<div style="text-align:center">
 <video autoplay muted loop>
  <source src="vid/main_event_loop_3.webm" type="video/webm">
  <p>The button now stops being responsive for 10 seconds after being pressed</p>
 </video>
</div>

What if the task is asynchronous by nature?
Let's try [`glib::timeout_future_seconds`](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/fn.timeout_future_seconds.html) as representation for our task instead of `std::thread::sleep`.
It returns a [`std::future::Future`](https://doc.rust-lang.org/std/future/trait.Future.html), which means we can `await` on it within an `async` context.
The converted code looks and behaves very similar to the multithreaded code.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/4/main.rs">listings/main_event_loop/4/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/4/main.rs:callback}}
```

Since we are single-threaded again, we can even get rid of the channel while achieving the same result.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/5/main.rs">listings/main_event_loop/5/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/5/main.rs:callback}}
```

But why did we not do the same thing with our multithreaded example?

```rust ,no_run,compile_fail
# use std::{thread, time::Duration};
#
# use glib::{clone, MainContext, PRIORITY_DEFAULT};
# use gtk::{glib, gio};
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow, Button};
#
# fn main() {
#     // Create a new application
#     let app = Application::builder()
#        .application_id("org.gtk_rs.MainEventLoop6")
#        .build();
#
#     // Connect to "activate" signal
#     app.connect_activate(build_ui);
#
#     // Get command-line arguments
#     let args: Vec<String> = args().collect();
#     // Run the application
#     app.run(&args);
# }
#
# // When the application is launched…
# fn build_ui(application: &Application) {
#     // Create a window
#     let window = ApplicationWindow::builder()
#         .application(application)
#         .title("My GTK App")
#         .build();
#
#     // Create a button
#     let button = Button::builder()
#         .label("Press me!")
#         .margin_top(12)
#         .margin_bottom(12)
#         .margin_start(12)
#         .margin_end(12)
#         .build();
#
    // DOES NOT COMPILE!
    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        button.clone();
        // The long running operation runs now in a separate thread
        gio::spawn_blocking(move || {
            // Deactivate the button until the operation is done
            button.set_sensitive(false);
            let five_seconds = Duration::from_secs(5);
            thread::sleep(five_seconds);
            // Activate the button again
            button.set_sensitive(true);
        });
    });
#
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```

Simply because we would get this error message:

```console
error[E0277]: `NonNull<GObject>` cannot be shared between threads safely

help: within `gtk4::Button`, the trait `Sync` is not implemented for `NonNull<GObject>`
```

After reference cycles we found the second disadvantage of GTK GObjects: They are not thread safe.

## Embed blocking calls in an `async` context

We've seen in the previous snippets that spawning an `async` block or `async` future on the `glib` main loop can lead to more concise code than running tasks on separate threads.
Let's focus on a few more aspects that are interesting to know when running `async` functions with gtk-rs apps.

For a start, blocking functions can be embedded within an `async` context.
In the following listing, we want to execute a synchronous function that returns a boolean and takes ten seconds to run.
In order to integrate it in our `async` block, we run the function in a separate thread via `spawn_blocking`.
We can then get the return value of the function by calling `await` on the return value of `spawn_blocking`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/6/main.rs">listings/main_event_loop/6/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/6/main.rs:callback}}
```

## Run `async` functions from external crates

Asynchronous functions from the `glib` ecosystem can always be spawned on the `glib` main loop.
Typically, crates depending on `async-std` or `smol` work as well.
Let us take `ashpd` for example which allows sandboxed applications to interact with the desktop.
It can be configured to depend on `async-std`.
We can add it to our dependencies by running the following command.

```
cargo add ashpd --no-default-features --features "gtk4 async-std"
```

You need to use a Linux desktop environment in order to run the following example locally.
This example is using [`ashpd::desktop::account::UserInformation`](https://docs.rs/ashpd/latest/ashpd/desktop/account/index.html) to access user information.
We are getting a [`gtk::Native`](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.Native.html) object from our button, create a [`ashpd::WindowIdentifier`](https://docs.rs/ashpd/latest/ashpd/enum.WindowIdentifier.html) and pass it to the user information request.

> We need to pass the `WindowIdentifier` to make the dialog modal. This means that it will be on top of the window and freezes the rest of the application from user input.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/7/main.rs">listings/main_event_loop/7/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/7/main.rs:callback}}
```

After pressing the button, a dialog should open that shows the information that will be shared.
If you decide to share it, you user name will be printed on the console.

<div style="text-align:center"><img src="img/main_event_loop_ashpd.png" alt="Dialog requesting user information."/></div>

## Tokio

[`tokio`](https://docs.rs/tokio/latest/tokio/) is Rust's most popular asynchronous platform.
Therefore, many high-quality crates are part of its ecosystem.
The web client [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) belongs to this group.
Let's add it by executing the following command

```
cargo add reqwest@0.12 --features rustls-tls --no-default-features
```

As soon as the button is pressed, we want to send a `GET` request to [www.gtk-rs.org](https://www.gtk-rs.org).
The response should then be sent to the main thread via a channel.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/8/main.rs">listings/main_event_loop/8/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/8/main.rs:callback}}
```

This compiles fine and even seems to run.
However, nothing happens when we press the button.
Inspecting the console gives the following error message:

```
thread 'main' panicked at
'there is no reactor running, must be called from the context of a Tokio 1.x runtime'
```

At the time of writing, `reqwest` doesn't document this requirement.
Unfortunately, that is also the case for other libraries depending on `tokio`.
Let's bite the bullet and add `tokio`:

```
cargo add tokio@1 --features rt-multi-thread
```

Since we already run the `glib` main loop on our main thread, we don't want to run the `tokio` runtime there.
For this reason, we **avoid** using the `#[tokio::main]` macro or using a top-level `block_on` call.
Doing this will block one of the runtime's threads with the GLib main loop, which is a waste of resources and a potential source of strange bugs.

Instead, we bind [`tokio::runtime::Runtime`](https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html) to a static variable.

```rust
# use std::sync::OnceLock;
#
# use glib::clone;
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow, Button};
# use tokio::runtime::Runtime;
#
# const APP_ID: &str = "org.gtk_rs.MainEventLoop0";
#
// DOES NOT COMPILE!
static RUNTIME: Runtime =
    Runtime::new().expect("Setting up tokio runtime needs to succeed.");
#
# fn main() -> glib::ExitCode {
#     // Create a new application
#     let app = Application::builder().application_id(APP_ID).build();
#
#     // Connect to "activate" signal of `app`
#     app.connect_activate(build_ui);
#
#     // Run the application
#     app.run()
# }
#
# fn build_ui(app: &Application) {
#    // Create a button
#    let button = Button::builder()
#        .label("Press me!")
#        .margin_top(12)
#        .margin_bottom(12)
#        .margin_start(12)
#        .margin_end(12)
#        .build();
#
#    // ANCHOR: callback
#    let (sender, receiver) = async_channel::bounded(1);
#    // Connect to "clicked" signal of `button`
#    button.connect_clicked(move |_| {
#        RUNTIME.spawn(clone!(#[strong] sender, async move {
#            let response = reqwest::get("https://www.gtk-rs.org").await;
#            sender.send(response).await.expect("The channel needs to be open.");
#        }));
#    });
#
#    // The main loop executes the asynchronous block
#    glib::spawn_future_local(async move {
#        while let Ok(response) = receiver.recv().await {
#            if let Ok(response) = response {
#                println!("Status: {}", response.status());
#            } else {
#                println!("Could not make a `GET` request.");
#            }
#        }
#    });
#    // ANCHOR_END: callback
#
#    // Create a window
#    let window = ApplicationWindow::builder()
#        .application(app)
#        .title("My GTK App")
#        .child(&button)
#        .build();
#
#    // Present window
#    window.present();
# }
```

Unfortunately, this doesn't compile.
As usual, Rust's error messages are really helpful.

```
cannot call non-const fn `tokio::runtime::Runtime::new` in statics
calls in statics are limited to constant functions, tuple structs and tuple variants
consider wrapping this expression in `Lazy::new(|| ...)` from the `once_cell` crate
```

We could follow the advice directly, but the standard library also provides solutions for that.
With [`std::sync::OnceLock`](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html) we can initialize the static with the const function `OnceLock::new()` and initialize it the first time our function `runtime` is called.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/9/main.rs">listings/main_event_loop/9/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/9/main.rs:tokio_runtime}}
```

In the button callback we can now spawn the `reqwest` `async` block with `tokio` rather than with `glib`.

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/main/book/listings/main_event_loop/9/main.rs">listings/main_event_loop/9/main.rs</a>

```rust
{{#rustdoc_include ../listings/main_event_loop/9/main.rs:callback}}
```

If we now press the button, we should find the following message in our console:

```
Status: 200 OK
```

We will not need `tokio`, `reqwest` or `ashpd` in the following chapters, so let's remove them again by executing:

```
cargo remove tokio reqwest ashpd
```

How to find out whether you can spawn an `async` task on the `glib` main loop?
`glib` should be able to spawn the task when the called functions come from libraries that either:

- come from the `glib` ecosystem,
- don't depend on a runtime but only on the `futures` family of crates (`futures-io`, `futures-core` etc),
- depend on the `async-std` or `smol` runtimes, or
- have cargo features that let them depend on `async-std`/`smol` instead of `tokio`.

## Conclusion

You don't want to block the main thread long enough that it is noticeable by the user.
But when should you spawn an `async` task, instead of spawning a task in a separate thread?
Let's go again through the different scenarios.

If the task spends its time calculating rather than waiting for a web response, it is [CPU-bound](https://en.wikipedia.org/wiki/CPU-bound).
That means you have to run the task in a separate thread and let it send results back via a channel.

If your task is [IO bound](https://en.wikipedia.org/wiki/I/O_bound), the answer depends on the crates at your disposal and the type of work to be done.

- Light I/O work with functions from crates using `glib`, `smol`, `async-std` or the `futures` trait family can be spawned on the main loop. This way, you can often avoid synchronization via channels.
- Heavy I/O work might still benefit from running in a separate thread / an async executor to avoid saturating the main loop. If you are unsure, benchmarking is advised.

If the best crate for the job relies on `tokio`, you will have to spawn it with the tokio runtime and communicate via channels.
