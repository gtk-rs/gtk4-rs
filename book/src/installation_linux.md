# Linux

## Flatpak

If you develop on Linux, using Flatpak is the most convenient option.
With Flatpak your whole workflow is containerized, and your users get the very same application you develop on including all dependencies. 
First, assure that Flatpak is installed on your system, check this [website](https://flatpak.org/setup/) to see if any steps are necessary on your distribution.
Download the [gtk-rust-template](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template) and follow the instructions in its README.

Then either install
- [GNOME Builder](https://flathub.org/apps/details/org.gnome.Builder) or
- [VSCodium](https://flathub.org/apps/details/com.vscodium.codium) together with the [rust-analyzer](https://open-vsx.org/extension/matklad/rust-analyzer) and [flatpak](https://open-vsx.org/extension/bilelmoussaoui/flatpak-vscode) extensions.

That is it.
The build dependencies can be downloaded by the IDE.
With GNOME Builder, you only have to press the run button for that.


## Host

If you develop on the host, you first have to install rustup.
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
