FROM fedora:latest

RUN dnf update -y
RUN dnf install git xorg-x11-server-Xvfb procps-ng wget libjpeg-turbo-devel expat-devel 'dnf-command(builddep)' -y
RUN dnf builddep gtk4 -y

# build gtk4 from the latest release
#ADD https://download.gnome.org/sources/gtk/4.6/gtk-4.6.0.tar.xz /tmp/gtk-4.6.0.tar.xz
#RUN tar -xf /tmp/gtk-4.6.0.tar.xz --directory /tmp
# WORKDIR /tmp/gtk-4.6.0
RUN git clone https://gitlab.gnome.org/gnOME/gtk.git --depth=1
WORKDIR gtk
RUN meson _build --prefix=/usr -Dgtk_doc=false -Dintrospection=disabled -Dbuild-examples=false -Dbuild-tests=false -Ddemos=false
RUN ninja -C _build
RUN ninja -C _build install
WORKDIR /
RUN rm -rf gtk
