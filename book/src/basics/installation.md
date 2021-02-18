## Flatpak (Linux only)

We recommend to use Flatpak for developing and packaging.
Flatpak offers a containerized workflow.
With it all dependencies and their versions are determined by you.
This frees you from worrying about which dependencies might be present on your system or the systems of your users.
Many distributions already include Flatpak; you can check [here](https://flatpak.org/setup/) if installations steps are necessary for you.

The easiest way to get started is to use [GNOME Builder](../ide/builder.html).

If you are already used to VSCode, you might be interested in the [flatpak-vscode](../ide/vscode.html) extension.

For other workflows, we recommend the [fenv](../build/fenv.html) CLI tool.


## Building on the Host System

You can also build on the host system if you prefer to do so.
First follow the instructions on the [GTK website](https://www.gtk.org/docs/installations/) to install GTK-4.

Then install Rust with the `rustup` tool
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Now create a new project with
```bash
cargo new my-gtk-project
```

Add the following lines to your `Cargo.toml` and you are ready to go!
```toml
[dependencies.gtk]
version = "0.1"
package = "gtk4"
```
