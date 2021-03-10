#!/bin/bash
set -x -e

# replace "gint" response_id parameters with "ResponseType"
xmlstarlet ed -L \
    -u '//_:parameter[@name="response_id"]/_:type[@name="gint"]/@c:type' -v "GtkResponseType" \
    -u '//_:parameter[@name="response_id"]/_:type[@name="gint"]/@name' -v "ResponseType" \
    Gtk-4.0.gir

# fix wrong "full" transfer ownership
xmlstarlet ed -L \
    -u '//_:constructor[@c:identifier="gtk_shortcut_label_new"]/_:return-value/@transfer-ownership' -v "none" \
    Gtk-4.0.gir

# fix non-existant c-types
xmlstarlet ed -L \
    -u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_keyboard"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_pointer"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandDevice"]/_:method[@name="get_wl_seat"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandDisplay"]/_:method[@name="get_wl_compositor"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandDisplay"]/_:method[@name="get_wl_display"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandMonitor"]/_:method[@name="get_wl_output"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandSeat"]/_:method[@name="get_wl_seat"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    -u '//_:class[@name="WaylandSurface"]/_:method[@name="get_wl_surface"]//_:type[@name="gpointer"]/@c:type' -v "gpointer" \
    GdkWayland-4.0.gir

# `///` used as `//` not works in Windows in this case
for file in *.gir; do
    xmlstarlet ed -L \
        -d '//_:doc/@line' \
        -d '//_:doc/@filename' \
        -d '///_:source-position' \
        "$file"
done
