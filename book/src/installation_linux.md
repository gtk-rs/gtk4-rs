# Linux

You first have to install rustup.
You can find the up-to-date instructions on [rustup.rs](https://rustup.rs).

Then install GTK 4, Libadwaita, Meson, and the build essentials.
To do this, execute the command belonging to the distribution you are using.

Fedora and derivatives:

```
sudo dnf install gtk4-devel libadwaita-devel meson desktop-file-utils gcc glib-compile-resources gtk4-update-icon-cache update-desktop-database
```

Debian and derivatives:

```
sudo apt install libgtk-4-dev libadwaita-1-dev meson desktop-file-utils gcc gtk-update-icon-cache
```

Arch and derivatives:

```
sudo pacman -S gtk4 libadwaita meson desktop-file-utils gcc
```
