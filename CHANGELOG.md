# Change Log

## [Unreleased]

## [0.6.1]

Julian Hofer:
- Update book to 0.6
- book: Fix clippy warnings

Mițca Dumitru:
- book: Adapt to glib-build-tools breaking change

Sebastian Dröge:
- gtk4: Use correct length for the `StrV` when passing to C in
  `ConstraintLayout::add_constraint_from_description()`

## [0.6.0]

Bilal Elmoussaoui:
- Add support for the to be released `gtk::UriLauncher`
- [Improve the API of `gtk::WidgetExt::dispose_template`](https://github.com/gtk-rs/gtk4-rs/pull/1212)
- [Mention the failed to retrieve template child name](https://github.com/gtk-rs/gtk4-rs/pull/1290)
- [Add a macos job](https://github.com/gtk-rs/gtk4-rs/pull/1237)

yuraiz:
- [Add blueprint support](https://github.com/gtk-rs/gtk4-rs/pull/1238)
```rust
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(string = "
template MyWidget : Widget {
    Label label {
        label: 'foobar';
    }
    Label my_label2 {
        label: 'foobaz';
    }
}
")]
pub struct MyWidget {
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    #[template_child(id = "my_label2")]
    pub label2: gtk::TemplateChild<gtk::Label>,
}
```


## 0.5.5

Bilal Elmoussaoui:

- [gtk: Generate FileLauncher API](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/98253e3f4ea7787b4ab7c705f379af5ac768e606)
- [gtk4-wayland: Bump wayland dependencies](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/619825d1985b420cb82a03ba3f58f2cb9c6bd0ad)
- [gtk4-macros: Bump quick-xml](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/ee63f8745603e6cd70cd34758c2901fe9f5ed25d)
- [gtk: Mark show_uri_full as deprecated](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/6a1e8b92410bf4a1b4da94b5354bdf811abfc982)
- [Regenerate with latest gir-files](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/cb917d096dafa08d2710376b1e4f3f2bad8f191b)

Maximilano: 

- [Mark new dialog api as not nullable](https://github.com/gtk-rs/gtk4-rs/pull/1233/commits/6b7ade231c90c676fc86351e86b52f99c2d5f104)

## 0.5.4

Bilal Elmoussaoui:

- [gtk: Subclass BuilderCScope for the BuilderRustScope](https://github.com/gtk-rs/gtk4-rs/pull/1217/commits/0c00d06c3f0f05362bb3bc8c7c4d78433970a78d)
- [Generate AlertDialog::choose](https://github.com/gtk-rs/gtk4-rs/pull/1217/commits/71f2266d5f0f78245cc54817bbba3ed916838b48)



## 0.5.3

Aaron Erhardt:

- [macros: Allow using re-exports of gtk](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/0d5b2c365a5736a00b2ae1b221e32446a91d3929)

Bilal Elmoussaoui:

- [gsk: Export builders module](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/5e6856b75337ae6f267f79b1c8938aaab189c102)
- [gtk: Properly mark deprecated manual items](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/7421e4714d9c5c1411a1190bf00dfe1d46e7df10)
- [gtk: Generate new 4.10 APIs](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/eabfc82d518f8b9d29452051f39c3209906355a2)
- [gtk: Fix new FontDialog APIs](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/2d4c19b6779d95df6256002e1dbc7798c6d9589b)
- [gtk: Generate AlertDialog](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/4f28a04e59ef8814ab3a858e42fe9d377c85fc5f)
- [gtk: Add IMContextImpl::activate_osk](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/0ba13215ba5f8c7aaed73a9e76f2a46ae45302d2)

Jason Francis:
- [gtk4: use impl_offset() for calculating template child offset](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/a3613c7b9b39fd6a93931e3d4fcbc2291e53272c)

Sebastian Dröge:
- [gtk4-macros: Update to quick-xml 0.26](https://github.com/gtk-rs/gtk4-rs/pull/1193/commits/064f8114cfa74a8d9d8ce644cd59cdc897d9ff35)

## 0.5.2

Marc-Andre Lureau:
- [gdk-win32: implement Win32Display.add_filter()](https://github.com/gtk-rs/gtk4-rs/pull/1174)

nardoor:
- [Skip init assertion for gdk::set_allowed_backends](https://github.com/gtk-rs/gtk4-rs/pull/1183)

## 0.5.1
Aaron Erhardt:
- [gtk: Add gnome_43 feature](https://github.com/gtk-rs/gtk4-rs/commit/ddbc370ff50b61e04157bee4cbc5d9e446db498d)
- [gtk: Add gnome_42 feature](https://github.com/gtk-rs/gtk4-rs/commit/05f692d5876a26ba23afc67057b87ed6cd7825e2)

Bilal Elmoussaoui:
- [gtk: Implement various traits for ResponseType](https://github.com/gtk-rs/gtk4-rs/commit/a270385868be03e50c4e8eb7286846c0de06095e)
- [gtk: Generate new v4.10 APIs](https://github.com/gtk-rs/gtk4-rs/commit/e70c71658479c022606389c26f33b0065d4a2148)

Marc-Andre Lureau:
- [Add gdk4-win32](https://github.com/gtk-rs/gtk4-rs/commit/159db780b3b2d6709c41cbdbe20f4b6088fd574a)
