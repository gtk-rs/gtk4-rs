# Linux

You first have to install rustup.
You can find the up-to-date instructions on [rustup.rs](https://rustup.rs).

Then install GTK 4 and the build essentials.
To do this, execute the command belonging to the distribution you are using.

Fedora and derivatives:

```
sudo dnf install gtk4-devel gcc
```

Debian and derivatives:

```
sudo apt install libgtk-4-dev build-essential
```

Arch and derivatives:

```
sudo pacman -S gtk4 base-devel
```
