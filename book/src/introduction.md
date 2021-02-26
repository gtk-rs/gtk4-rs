# The GTK-Rust book

*by Julian Hofer, Bilal Elmoussaoui & Christopher Davis with contributions from the community*

Welcome to the GTK-Rust book, a book about developing graphical applications using GTK and Rust.
GTK is a cross-platform widget toolkit which supports a wide range of programming languages.
Thanks to GObject-Introspection, GTK's API even describes the ownership of its parameters.
Managing ownership while remaining low-level is one of Rust's greatest strengths, which makes it an excellent choice to develop GTK apps with it.
With this combination you don't have to worry about hitting bottlenecks mid-project anymore.
Additionally, with Rust you'll have nice things like:
 - Thread safety
 - Memory safety
 - Sensible dependency management
 - Excellent third party libraries, which benefit from the same points as mentioned above


## Who this Book is for

This book assumes that you know your way around Rust code.
If this is not already the case, reading [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) is an enjoyable way to get you to that stage.
If you have experience with another low-level language such as C or C++ you
might find that reading [A half hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) gives you sufficient information as well.

Luckily, this together with the wish to develop graphical applications is all that's necessary to benefit from this book. 


## How to use this Book

In general, this book assumes that you’re reading it in sequence from front to
back. However, if you are using it as a reference for a certain topic,
you might find it useful to just jump into it.

There are two kinds of chapters in this book: concept chapters and project
chapters. In concept chapters, you’ll learn about an aspect of GTK development. In project
chapters, we’ll build small programs together, applying what you’ve learned so
far. 

*The GTK-Rust book is heavily inspired by [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)*
