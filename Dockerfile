FROM ghcr.io/gtk-rs/gtk-rs-core/core:latest

RUN dnf update -y && \
    dnf install xorg-x11-server-Xvfb procps-ng \
    libxkbcommon-devel wayland-devel mesa-libEGL-devel \
    libXi-devel libXrandr-devel libXcursor-devel libXdamage-devel libXinerama-devel -y && \
    dnf clean all -y

RUN git clone https://gitlab.gnome.org/gnome/gtk.git --depth=1 && \
    (cd /gtk && \
        meson setup builddir --prefix=/usr --buildtype release -Dgtk_doc=false -Dintrospection=disabled -Dbuild-examples=false -Dbuild-tests=false -Ddemos=false -Dmedia-gstreamer=disabled -Dlibepoxy:tests=false && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/libadwaita.git --depth=1 -b libadwaita-1-2 && \
    (cd /libadwaita && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=disabled -Dvapi=false -Dexamples=false -Dtests=false && \
        meson install -C builddir) && \
    rm -rf /gtk /libadwaita
