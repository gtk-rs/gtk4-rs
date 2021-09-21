FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng wget 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
ADD https://download-fallback.gnome.org/sources/gtk/4.4/gtk-4.4.0.tar.xz /tmp/gtk-4.4.0.tar.xz
RUN tar -xf /tmp/gtk-4.4.0.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.4.0
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
