FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
ADD https://download.gnome.org/sources/gtk/4.1/gtk-4.1.2.tar.xz /tmp/gtk-4.1.2.tar.xz
RUN tar -xf /tmp/gtk-4.1.2.tar.xz --directory /tmp
WORKDIR /tmp/gtk-4.1.2
RUN meson _build --prefix=/usr
RUN ninja -C _build
RUN ninja -C _build install
