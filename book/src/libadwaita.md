# Libadwaita

If you target a certain platform with your GUI, you will want to follow the platform's [Human Interface Guidelines](https://en.wikipedia.org/wiki/Human_interface_guidelines) (HIG).
With a GTK application, chances are the platform is either [elementary OS](https://elementary.io) or [GNOME](https://www.gnome.org).
In this chapter we will discuss how to follow GNOME's [HIG](https://developer.gnome.org/hig/) with [libadwaita](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/). 

Libadwaita is a library augmenting GTK 4 which:
- provides widgets to better follow GNOME's HIG
- provides widgets to let applications [change their layout ](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/adaptive-layouts.html) based on the available space
- integrates the Adwaita [stylesheet](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/styles-and-appearance.html)
- allows runtime recoloring with [CSS variables](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/css-variables.html#ui-colors)
- adds [API](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/docs/libadwaita/struct.StyleManager.html) to support the cross-desktop dark style preference

In order to use the Rust bindings, add the [libadwaita crate](https://crates.io/crates/libadwaita) as dependency by executing:

```
cargo add libadwaita --rename adw --features v1_5
```

The versions of the `gtk4` and `libadwaita` crates need to be synced.
Just remember that when you update one of them to the newest version to update the other one as well. 

Installation of the library itself works similar to GTK.
Just follow the installation instruction that is suitable for your distribution.

## Linux

Fedora and derivatives:

```
sudo dnf install libadwaita-devel
```

Debian and derivatives:

```
sudo apt install libadwaita-1-dev
```

Arch and derivatives:

```
sudo pacman -S libadwaita
```

## macOS

```
brew install libadwaita
```

## Windows

## If using gvsbuild

If you used `gvsbuild` to build GTK 4:

```
gvsbuild build libadwaita librsvg
```


## If building manually with MSVC:

From the Windows start menu, search for `x64 Native Tools Command Prompt for VS 2019`.
That will open a terminal configured to use MSVC x64 tools.
From there, run the following commands:

```
cd /
git clone --branch libadwaita-1-3 https://gitlab.gnome.org/GNOME/libadwaita.git --depth 1
cd libadwaita
meson setup builddir -Dprefix=C:/gnome -Dintrospection=disabled -Dvapi=false
meson install -C builddir
```

## Work around missing icons

[This workaround is needed for GTK < 4.10](https://gitlab.gnome.org/GNOME/gtk/-/blob/34b9ec5be2f3a38e1e72c4d96f130a2b14734121/NEWS#L60)
due to [this issue](https://gitlab.gnome.org/GNOME/gtk/-/issues/5303).

### gvsbuild

From a command prompt:

```
xcopy /s /i C:\gtk-build\gtk\x64\release\share\icons\hicolor\scalable\apps C:\gtk-build\gtk\x64\release\share\icons\hicolor\scalable\actions
gtk4-update-icon-cache.exe -t -f C:\gtk-build\gtk\x64\release\share\icons\hicolor
```

### Manually with MSVC


```
xcopy /s /i C:\gnome\share\icons\hicolor\scalable\apps C:\gnome\share\icons\hicolor\scalable\actions
gtk4-update-icon-cache.exe -t -f C:\gnome\share\icons\hicolor
```
