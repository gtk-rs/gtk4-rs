// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`ColorChooser`](crate::ColorChooser) interface.

use crate::{prelude::*, subclass::prelude::*, ColorChooser, Orientation};
use gdk::RGBA;
use glib::translate::*;

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ColorChooserImpl: ObjectImpl {
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        self.parent_add_palette(orientation, colors_per_line, colors);
    }

    fn color_activated(&self, rgba: RGBA) {
        self.parent_color_activated(rgba);
    }

    #[doc(alias = "get_rgba")]
    fn rgba(&self) -> RGBA;
    fn set_rgba(&self, rgba: RGBA);
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait ColorChooserImplExt: ObjectSubclass {
    fn parent_add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]);
    fn parent_color_activated(&self, rgba: RGBA);
    fn parent_rgba(&self) -> RGBA;
    fn parent_set_rgba(&self, rgba: RGBA);
}

impl<T: ColorChooserImpl> ColorChooserImplExt for T {
    fn parent_add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: &[RGBA]) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ColorChooser>()
                as *const ffi::GtkColorChooserInterface;

            if let Some(func) = (*parent_iface).add_palette {
                let colors_ptr: Vec<gdk::ffi::GdkRGBA> =
                    colors.iter().map(|c| *c.to_glib_none().0).collect();

                func(
                    self.obj()
                        .unsafe_cast_ref::<ColorChooser>()
                        .to_glib_none()
                        .0,
                    orientation.into_glib(),
                    colors_per_line,
                    colors.len() as i32,
                    mut_override(colors_ptr.as_ptr()),
                )
            }
        }
    }

    fn parent_color_activated(&self, rgba: RGBA) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ColorChooser>()
                as *const ffi::GtkColorChooserInterface;

            if let Some(func) = (*parent_iface).color_activated {
                func(
                    self.obj()
                        .unsafe_cast_ref::<ColorChooser>()
                        .to_glib_none()
                        .0,
                    rgba.to_glib_none().0,
                )
            }
        }
    }

    fn parent_rgba(&self) -> RGBA {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ColorChooser>()
                as *const ffi::GtkColorChooserInterface;

            let func = (*parent_iface)
                .get_rgba
                .expect("no parent \"get_rgba\" implementation");
            let rgba = std::ptr::null_mut();
            func(
                self.obj()
                    .unsafe_cast_ref::<ColorChooser>()
                    .to_glib_none()
                    .0,
                rgba,
            );
            from_glib_none(rgba)
        }
    }

    fn parent_set_rgba(&self, rgba: RGBA) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<ColorChooser>()
                as *const ffi::GtkColorChooserInterface;

            if let Some(func) = (*parent_iface).set_rgba {
                func(
                    self.obj()
                        .unsafe_cast_ref::<ColorChooser>()
                        .to_glib_none()
                        .0,
                    rgba.to_glib_none().0,
                )
            }
        }
    }
}

unsafe impl<T: ColorChooserImpl> IsImplementable<T> for ColorChooser {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.add_palette = Some(color_chooser_add_palette::<T>);
        iface.color_activated = Some(color_chooser_color_activated::<T>);
        iface.get_rgba = Some(color_chooser_get_rgba::<T>);
        iface.set_rgba = Some(color_chooser_set_rgba::<T>);
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
    let imp = instance.imp();

    let colors = if total == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(colorsptr as *const RGBA, total as usize)
    };
    imp.add_palette(from_glib(orientation), colors_per_line, colors);
}

unsafe extern "C" fn color_chooser_color_activated<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgba: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.imp();

    imp.color_activated(from_glib_none(rgba))
}

unsafe extern "C" fn color_chooser_get_rgba<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgbaptr: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.imp();

    let rgba = imp.rgba();
    *(rgbaptr as *mut gdk::ffi::GdkRGBA) = *rgba.to_glib_none().0;
}

unsafe extern "C" fn color_chooser_set_rgba<T: ColorChooserImpl>(
    color_chooser: *mut ffi::GtkColorChooser,
    rgba: *const gdk::ffi::GdkRGBA,
) {
    let instance = &*(color_chooser as *mut T::Instance);
    let imp = instance.imp();

    imp.set_rgba(from_glib_none(rgba))
}
