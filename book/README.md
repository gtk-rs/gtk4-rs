# The GTK-Rust Book

At the moment, it is deployed [here](https://hofer-julian.pages.gitlab.gnome.org/gtk-rs-book).

## Build instructions

1. Install mdbook

```bash
$ cargo install mdbook
```

2. Building

To view the book, execute:

```bash
$ mdbook serve
```

## Listings

Go to the listings folder

```bash
$ cd listings
```

To check if the examples build, execute:

```bash
$ cargo build
```

To run a specific listing, execute:

```bash
$ cargo run --bin listing_name
```

## License

The book itself is licensed under the [Creative Commons Attribution 3.0 Unported license](https://creativecommons.org/licenses/by/3.0/).
One exception are the code snippets which are licensed under the [MIT license](LICENSE.md).