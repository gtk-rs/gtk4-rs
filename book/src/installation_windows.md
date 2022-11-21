# Windows

When preparing your Windows machine, you have to decide between either using the **MSVC toolchain** or the **GNU toolchain**.
If in doubt, go for MSVC since that is the default on Windows.
You will want to go for the GNU toolchain if you depend on libraries that can only be compiled with the GNU toolchain.


## MSVC toolchain

### Install Rustup

Install the rust toolchain via [rustup](https://rustup.rs/).

### Remove residues from the GNU toolchain

If you used the GNU toolchain before, make sure to revert all changes you made to environment variables during the installation process.

Also set the rust toolchain back to msvc by executing:

```
rustup default stable-msvc
```

### Visual Studio

Install Visual Studio Community from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/de/vs/community/).
Make sure to check the box "Desktop development with C++" during the installation process.

<div style="text-align:center"><img src="img/vs-install.png" /></div>

### Git

Download git from [gitforwindows.org](https://gitforwindows.org/).


### Python

Download python from [python.org](https://www.python.org/downloads).
Make sure to opt-in to adding Python to your Path during the installation process.


### Meson

Install meson by executing:

```powershell
pip install meson ninja
```


### Pkg-config

Download pkg-config-lite from [sourceforge.net](https://sourceforge.net/projects/pkgconfiglite/).
Then extract and unpack it in `C:/`, so that the executable is in `C:\pkg-config-lite-0.28-1\bin`.


### Update `Path` environment variable

1. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`
2. Select `Path` -> Click on `Edit` -> Add the following entries:
 
```
C:\pkg-config-lite-0.28-1\bin
C:\gnome\bin
```

### Compile and install GTK 4

From the Windows start menu, search for `x64 Native Tools Command Prompt for VS 2019`.
That will open a terminal configured to use MSVC x64 tools.
From there, run the following commands:

```powershell
cd /
git clone https://gitlab.gnome.org/GNOME/gtk.git --depth 1
cd gtk
meson setup builddir --prefix=C:/gnome -Dbuild-tests=false -Dmedia-gstreamer=disabled
meson install -C builddir
```

### Set `PKG_CONFIG_PATH` environment variable

1. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`
2. Under `User variables` click on `New` and add:

- Variable name: `PKG_CONFIG_PATH`
- Variable value: `C:\gnome\lib\pkgconfig`


## GNU toolchain

### Install Rustup

Install the rust toolchain via [rustup](https://rustup.rs/).

### Remove residues from the MSVC toolchain

If you used the MSVC toolchain before, make sure to revert all changes you made to environment variables during the installation process.

### MSYS2

Install MSYS2 from [www.msys2.org](https://www.msys2.org/) 

### Install GTK 4

From the Windows start menu, search for `MSYS2 MinGW 64-bit`.
That will open a terminal configured to use MinGW x64 tools.

There, execute the following commands to install `GTK 4`, `pkgconf` and `gcc`.

```sh
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc
```


### Update `Path` environment variable

1. Go to settings -> Search and open `Advanced system settings` -> Click on `Environment variables`
2. Select `Path` -> Click on `Edit` -> Add the following three entries:
 
```
C:\msys64\mingw64\include
C:\msys64\mingw64\bin
C:\msys64\mingw64\lib
```

### Setup the GNU toolchain for Rust

The default toolchain on windows is `stable-msvc`.
To switch to `stable-gnu`, run the following commands from your terminal:

1. `rustup toolchain install stable-gnu`
2. `rustup default stable-gnu`

Please note that this command might change in the future.
If it does not work anymore, please open an [issue](https://github.com/gtk-rs/gtk4-rs/issues/new/choose) on our repo.
