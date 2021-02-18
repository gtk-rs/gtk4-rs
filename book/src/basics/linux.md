# Linux

## Flatpak

We recommend using Flatpak for developing and packaging on Linux.
Flatpak offers a containerized workflow, all dependencies and its versions are determined by you.
This frees you from worrying, which dependencies might be present on your system or the system of your users.

The easiest way to get started is to use GNOME Builder.

If you are already used to VSCode, you might be interested in the flatpak-vscode extension.

For other workflows, we recommend the fenv CLI tool.


## Package manager

**TODO: Check if these instructions actually work**

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



**TO-DO: How to add gtk4 to Cargo.toml as soon as it is released**
