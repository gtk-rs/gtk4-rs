FROM ghcr.io/gtk-rs/gtk-rs-core/core:latest

RUN dnf update -y && \
    dnf install xorg-x11-server-Xvfb procps-ng clang-devel \
    libxkbcommon-devel wayland-devel mesa-libEGL-devel blueprint-compiler \
    libXi-devel libXrandr-devel libXcursor-devel libXdamage-devel libXinerama-devel libdrm-devel -y && \
    dnf clean all -y

RUN git clone https://gitlab.gnome.org/gnome/gtk.git --depth=1 && \
    (cd /gtk && \
        meson setup builddir --prefix=/usr --buildtype release -Dgtk_doc=false -Dintrospection=enabled -Dbuild-examples=false -Dbuild-tests=false -Ddemos=false -Dmedia-gstreamer=disabled -Dlibepoxy:tests=false && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/libadwaita.git --depth=1 --branch libadwaita-1-4 && \
    (cd /libadwaita && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=disabled -Dvapi=false -Dexamples=false -Dtests=false && \
        meson install -C builddir) && \
    rm -rf /gtk /libadwaita
