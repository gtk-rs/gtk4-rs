# GUI development with Rust and GTK 4
*by Julian Hofer, with contributions from the community*

GTK 4 is the newest version of a popular cross-platform widget toolkit written in C.
Thanks to GObject-Introspection, GTK's API can be easily targeted by various programming languages.
The API even describes the ownership of its parameters!

Managing ownership without giving up speed is one of Rust's greatest strengths, which makes it an excellent choice to develop GTK apps with.
With this combination you don't have to worry about hitting bottlenecks mid-project anymore.
Additionally, with Rust you will have nice things such as
 - thread safety,
 - memory safety,
 - sensible dependency management as well as
 - excellent third party libraries.

The [`gtk-rs`](https://gtk-rs.org/) project provides bindings to many GTK-related libraries which we will be using throughout this book.


## Who this book is for

This book assumes that you know your way around Rust code.
If this is not already the case, reading [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) is an enjoyable way to get you to that stage.
If you have experience with another low-level language such as C or C++ you
might find that reading [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) gives you sufficient information as well.

Luckily, this — together with the wish to develop graphical applications — is all that is necessary to benefit from this book. 


## How to use this book

In general, this book assumes that you are reading it in sequence from front to
back. However, if you are using it as a reference for a certain topic,
you might find it useful to just jump into it.

There are two kinds of chapters in this book: concept chapters and project
chapters.
In concept chapters, you will learn about an aspect of GTK development.
In project chapters, we will build small programs together, applying what you've learned so far.

The book strives to explain essential GTK concepts paired with practical examples.
However, if a concept can be better conveyed with a less practical example, we took this path most of the time.
If you are interested in contained and useful examples, we refer you to the corresponding section of `gtk4-rs`' [repository](https://github.com/gtk-rs/gtk4-rs/tree/main/examples).

Every valid code snippet in the book is part of a listing.
Like the examples, the listings be found in the [repository](https://github.com/gtk-rs/gtk4-rs/tree/main/book/listings) of `gtk4-rs`.

## Translations

This book has been translated to Chinese by 陈竞阁 and is served under the following [website](https://mario-hero.github.io/gtk-book-zh_cn/).

## License

The book itself is licensed under the [Creative Commons Attribution 4.0 International license](https://creativecommons.org/licenses/by/4.0/).
The only exception are the code snippets which are licensed under the [MIT license](https://github.com/gtk-rs/gtk4-rs/blob/main/README.md).
