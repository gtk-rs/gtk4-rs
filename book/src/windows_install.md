# Installing GTK4 on Windows

Installing GTK4 for gtk4-rs on Windows just takes a few minutes.
Follow the **preparation** first and then proceed with installing GTK either for the **GNU toolchain**
or the **MSVC toolchain**.

## Preparation

1. Install Rust using [rustup](https://rustup.rs)
2. Install MSYS2 from [www.msys2.org](https://www.msys2.org/) 

### Update path environment variable

1. Go to settings -> Search and open `Advanced system settings` -> click on `Environment variables`
2. Select `Path` -> Click on `Edit` -> Add the following three entries:
 
```
C:\msys64\mingw64\include
C:\msys64\mingw64\bin
C:\msys64\mingw64\lib
```


## GNU toolchain

Using the GNU toolchain is usually the easier method and recommended unless you know you want to use MSVC instead.

### Setup the `windows-gnu` toolchain for Rust

Run the following commands from your terminal:

1. `rustup toolchain install stable-gnu`
2. `rustup default stable-gnu`

### Install GTK

Run the following command from your **MSYS2 terminal**:

```sh
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkg-config mingw-w64-x86_64-gcc
```


## MSVC toolchain

### Install required tools
1. Install [Visual Studio](https://visualstudio.microsoft.com/vs/) with C++ compiler tools
2. Install git from [git-scm.com](https://git-scm.com/download/win)
3. Install meson from [github.com/mesonbuild/meson](https://github.com/mesonbuild/meson/releases)
4. Install python from [www.python.org](https://www.python.org/downloads/)

### Install pkg-config

Run the following command from your **MSYS2 terminal**:

```sh
pacman -S pkg-config
```

### Compile and install GTK4

Run the following commands from your terminal:

```sh
git clone https://gitlab.gnome.org/GNOME/gtk.git
cd gtk
meson setup build -Dmedia-gstreamer=disabled -Dbuild-tests=false
meson compile -C build
meson install -C build
```
