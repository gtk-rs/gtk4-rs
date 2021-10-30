FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng wget libjpeg-turbo-devel 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/4.5/gtk-4.5.0.tar.xz /tmp/gtk-4.5.0.tar.xz
RUN tar -xf /tmp/gtk-4.5.0.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.5.0
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
