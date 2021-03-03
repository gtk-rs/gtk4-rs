# Introduction

Welcome to *GUI development with Rust and GTK 4*.

GTK 4 is the newest version of a popular cross-platform widget toolkit.
Thanks to the information provided by GObject-Introspection, GTK's API can be easily target by various programming languages.
It even describes the ownership of its parameters!

Managing ownership without giving up speed is one of Rust's greatest strengths, which makes it an excellent choice to develop GTK apps with it.
With this combination you don't have to worry about hitting bottlenecks mid-project anymore.
Additionally, with Rust you'll have nice things like:
 - Thread safety
 - Memory safety
 - Sensible dependency management
 - Excellent third party libraries, which benefit from the same points as mentioned above


## Who this book is for

This book assumes that you know your way around Rust code.
If this is not already the case, reading [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) is an enjoyable way to get you to that stage.
If you have experience with another low-level language such as C or C++ you
might find that reading [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) gives you sufficient information as well.

Luckily, this together with the wish to develop graphical applications is all that's necessary to benefit from this book. 


## How to use this book

In general, this book assumes that you’re reading it in sequence from front to
back. However, if you are using it as a reference for a certain topic,
you might find it useful to just jump into it.

There are two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of GTK development. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. 

## License

The book itself is licensed under the [Creative Commons Attribution 3.0 Unported license](https://creativecommons.org/licenses/by/3.0/).
One exception are the code snippets which are licensed under the [MIT license](https://gitlab.gnome.org/Hofer-Julian/gtk-rs-book/-/blob/main/LICENSE).
