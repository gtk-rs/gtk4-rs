FROM ghcr.io/gtk-rs/gtk-rs-core/core:latest

RUN dnf update --assumeyes && \
    dnf --assumeyes install xorg-x11-server-Xvfb procps-ng clang-devel \
    libxkbcommon-devel wayland-devel mesa-libEGL-devel blueprint-compiler \
    libXi-devel libXrandr-devel libXcursor-devel libXdamage-devel libXinerama-devel \
    appstream-devel libdrm-devel vulkan-devel glslc awk && \
    dnf clean all --assumeyes

RUN git clone https://gitlab.gnome.org/gnome/gtk.git --depth=1 && \
    (cd /gtk && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=enabled -Dbuild-examples=false -Dbuild-tests=false -Dmedia-gstreamer=disabled -Dlibepoxy:tests=false && \
        meson install -C builddir) && \
    git clone https://gitlab.gnome.org/GNOME/libadwaita.git --depth=1 --branch libadwaita-1-7 && \
    (cd /libadwaita && \
        meson setup builddir --prefix=/usr --buildtype release -Dintrospection=disabled -Dvapi=false -Dexamples=false -Dtests=false && \
        meson install -C builddir) && \
    rm -rf /gtk /libadwaita
