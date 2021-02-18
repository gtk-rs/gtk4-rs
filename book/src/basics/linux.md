# Linux

## Flatpak

We recommend to use Flatpak for developing and packaging on Linux.
Flatpak offers a containerized workflow.
With it all dependencies and its versions are determined by you.
This frees you from worrying, which dependencies might be present on your system or the system of your users.
Many distributions already include Flatpak, you can check [here](https://flatpak.org/setup/ work) if installations steps are necessary for you.

The easiest way to get started is to use [GNOME Builder](../ide/builder.html).

If you are already used to VSCode, you might be interested in the [flatpak-vscode)(../ide/vscode.html) extension.

For other workflows, we recommend the [fenv](../build/fenv.html) CLI tool.


## Package manager

If you just want to follow the examples in the book you can also the package manager of your distribution.
This of course requires that your distribution already packages GTK-4.

If you use Fedora, execute:
```bash
sudo dnf install rust cargo gtk4-devel
```

If you use Debian or a derivative such as Ubuntu, execute:
```bash
sudo apt install rustc cargo libgtk-4-dev
```

Now create a new project with
```bash
cargo new my-gtk-project
```

Add the following line to your `Cargo.toml` and you are ready to go!
```toml
gtk4-rs = "0.1"
```
