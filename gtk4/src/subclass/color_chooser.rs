// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ColorChooser, Orientation};
use gdk::RGBA;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait ColorChooserImpl: ObjectImpl {
    fn add_palette(
        &self,
        color_chooser: &Self::Type,
        orientation: Orientation,
        colors_per_line: i32,
        colors: &[RGBA],
    );
    fn color_activated(&self, color_chooser: &Self::Type, rgba: RGBA);
    fn get_rgba(&self, color_chooser: &Self::Type) -> RGBA;
    fn set_rgba(&self, color_chooser: &Self::Type, rgba: RGBA);
}

unsafe impl<T: ColorChooserImpl> IsImplementable<T> for ColorChooser {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let color_chooser_iface = &mut *(iface as *mut ffi::GtkColorChooserInterface);

        color_chooser_iface.add_palette = Some(color_chooser_add_palette::<T>);
        color_chooser_iface.color_activated = Some(color_chooser_color_activated::<T>);
        color_chooser_iface.get_rgba = Some(color_chooser_get_rgba::<T>);
        color_chooser_iface.set_rgba = Some(color_chooser_set_rgba::<T>);
    }
}

unsafe extern "C" fn color_chooser_add_palette<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    orientation: ffi::GtkOrientation,
    colors_per_line: i32,
    total: i32,
    colorsptr: *mut gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    let colors = std::slice::from_raw_parts(colorsptr as *const RGBA, total as usize);
    imp.add_palette(
        from_glib_borrow::<_, ColorChooser>(color_chooser).unsafe_cast_ref(),
        from_glib(orientation),
        colors_per_line,
        &colors,
    );
}

unsafe extern "C" fn color_chooser_color_activated<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgba: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    imp.color_activated(
        from_glib_borrow::<_, ColorChooser>(color_chooser).unsafe_cast_ref(),
        from_glib_none(rgba),
    )
}

unsafe extern "C" fn color_chooser_get_rgba<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgbaptr: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    let rgba = imp.get_rgba(from_glib_borrow::<_, ColorChooser>(color_chooser).unsafe_cast_ref());
    *(rgbaptr as *mut gdk::ffi::GdkRGBA) = *rgba.to_glib_none().0;
}

unsafe extern "C" fn color_chooser_set_rgba<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgba: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    imp.set_rgba(
        from_glib_borrow::<_, ColorChooser>(color_chooser).unsafe_cast_ref(),
        from_glib_none(rgba),
    )
}
