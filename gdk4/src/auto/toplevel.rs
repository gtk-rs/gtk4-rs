// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
use crate::TitlebarGesture;
use crate::{
    ffi, Device, Event, FullscreenMode, Surface, SurfaceEdge, Texture, ToplevelLayout,
    ToplevelState,
};
#[cfg(feature = "v4_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
use crate::{Gravity, ToplevelCapabilities};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkToplevel")]
    pub struct Toplevel(Interface<ffi::GdkToplevel, ffi::GdkToplevelInterface>) @requires Surface;

    match fn {
        type_ => || ffi::gdk_toplevel_get_type(),
    }
}

impl Toplevel {
    pub const NONE: Option<&'static Toplevel> = None;
}

pub trait ToplevelExt: IsA<Toplevel> + 'static {
    #[doc(alias = "gdk_toplevel_begin_move")]
    fn begin_move(&self, device: &impl IsA<Device>, button: i32, x: f64, y: f64, timestamp: u32) {
        unsafe {
            ffi::gdk_toplevel_begin_move(
                self.as_ref().to_glib_none().0,
                device.as_ref().to_glib_none().0,
                button,
                x,
                y,
                timestamp,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_begin_resize")]
    fn begin_resize(
        &self,
        edge: SurfaceEdge,
        device: Option<&impl IsA<Device>>,
        button: i32,
        x: f64,
        y: f64,
        timestamp: u32,
    ) {
        unsafe {
            ffi::gdk_toplevel_begin_resize(
                self.as_ref().to_glib_none().0,
                edge.into_glib(),
                device.map(|p| p.as_ref()).to_glib_none().0,
                button,
                x,
                y,
                timestamp,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_focus")]
    fn focus(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_toplevel_focus(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gdk_toplevel_get_capabilities")]
    #[doc(alias = "get_capabilities")]
    fn capabilities(&self) -> ToplevelCapabilities {
        unsafe {
            from_glib(ffi::gdk_toplevel_get_capabilities(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gdk_toplevel_get_gravity")]
    #[doc(alias = "get_gravity")]
    fn gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::gdk_toplevel_get_gravity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_toplevel_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> ToplevelState {
        unsafe { from_glib(ffi::gdk_toplevel_get_state(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_toplevel_inhibit_system_shortcuts")]
    fn inhibit_system_shortcuts(&self, event: Option<impl AsRef<Event>>) {
        unsafe {
            ffi::gdk_toplevel_inhibit_system_shortcuts(
                self.as_ref().to_glib_none().0,
                event.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_lower")]
    fn lower(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_lower(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_toplevel_minimize")]
    fn minimize(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_minimize(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_toplevel_present")]
    fn present(&self, layout: &ToplevelLayout) {
        unsafe {
            ffi::gdk_toplevel_present(self.as_ref().to_glib_none().0, layout.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_toplevel_restore_system_shortcuts")]
    fn restore_system_shortcuts(&self) {
        unsafe {
            ffi::gdk_toplevel_restore_system_shortcuts(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_toplevel_set_decorated")]
    #[doc(alias = "decorated")]
    fn set_decorated(&self, decorated: bool) {
        unsafe {
            ffi::gdk_toplevel_set_decorated(self.as_ref().to_glib_none().0, decorated.into_glib());
        }
    }

    #[doc(alias = "gdk_toplevel_set_deletable")]
    #[doc(alias = "deletable")]
    fn set_deletable(&self, deletable: bool) {
        unsafe {
            ffi::gdk_toplevel_set_deletable(self.as_ref().to_glib_none().0, deletable.into_glib());
        }
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gdk_toplevel_set_gravity")]
    #[doc(alias = "gravity")]
    fn set_gravity(&self, gravity: Gravity) {
        unsafe {
            ffi::gdk_toplevel_set_gravity(self.as_ref().to_glib_none().0, gravity.into_glib());
        }
    }

    #[doc(alias = "gdk_toplevel_set_icon_list")]
    #[doc(alias = "icon-list")]
    fn set_icon_list(&self, surfaces: &[Texture]) {
        unsafe {
            ffi::gdk_toplevel_set_icon_list(
                self.as_ref().to_glib_none().0,
                surfaces.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_set_modal")]
    #[doc(alias = "modal")]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gdk_toplevel_set_modal(self.as_ref().to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "gdk_toplevel_set_startup_id")]
    #[doc(alias = "startup-id")]
    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_toplevel_set_startup_id(
                self.as_ref().to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_set_title")]
    #[doc(alias = "title")]
    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gdk_toplevel_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_toplevel_set_transient_for")]
    #[doc(alias = "transient-for")]
    fn set_transient_for(&self, parent: &impl IsA<Surface>) {
        unsafe {
            ffi::gdk_toplevel_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_show_window_menu")]
    fn show_window_menu(&self, event: impl AsRef<Event>) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_show_window_menu(
                self.as_ref().to_glib_none().0,
                event.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_toplevel_supports_edge_constraints")]
    fn supports_edge_constraints(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_supports_edge_constraints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_toplevel_titlebar_gesture")]
    fn titlebar_gesture(&self, gesture: TitlebarGesture) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_titlebar_gesture(
                self.as_ref().to_glib_none().0,
                gesture.into_glib(),
            ))
        }
    }

    fn is_decorated(&self) -> bool {
        ObjectExt::property(self.as_ref(), "decorated")
    }

    fn is_deletable(&self) -> bool {
        ObjectExt::property(self.as_ref(), "deletable")
    }

    #[doc(alias = "fullscreen-mode")]
    fn fullscreen_mode(&self) -> FullscreenMode {
        ObjectExt::property(self.as_ref(), "fullscreen-mode")
    }

    #[doc(alias = "fullscreen-mode")]
    fn set_fullscreen_mode(&self, fullscreen_mode: FullscreenMode) {
        ObjectExt::set_property(self.as_ref(), "fullscreen-mode", fullscreen_mode)
    }

    //#[doc(alias = "icon-list")]
    //fn icon_list(&self) -> /*Unimplemented*/Basic: Pointer {
    //    ObjectExt::property(self.as_ref(), "icon-list")
    //}

    fn is_modal(&self) -> bool {
        ObjectExt::property(self.as_ref(), "modal")
    }

    #[doc(alias = "shortcuts-inhibited")]
    fn is_shortcuts_inhibited(&self) -> bool {
        ObjectExt::property(self.as_ref(), "shortcuts-inhibited")
    }

    #[doc(alias = "startup-id")]
    fn startup_id(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "startup-id")
    }

    fn title(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "title")
    }

    #[doc(alias = "transient-for")]
    fn transient_for(&self) -> Option<Surface> {
        ObjectExt::property(self.as_ref(), "transient-for")
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "capabilities")]
    fn connect_capabilities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_capabilities_trampoline<
            P: IsA<Toplevel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::capabilities".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_capabilities_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "decorated")]
    fn connect_decorated_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_decorated_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::decorated".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_decorated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "deletable")]
    fn connect_deletable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_deletable_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::deletable".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_deletable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fullscreen-mode")]
    fn connect_fullscreen_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fullscreen_mode_trampoline<
            P: IsA<Toplevel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::fullscreen-mode".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_fullscreen_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_20")))]
    #[doc(alias = "gravity")]
    fn connect_gravity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gravity_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::gravity".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_gravity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-list")]
    fn connect_icon_list_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_list_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::icon-list".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_list_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::modal".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shortcuts-inhibited")]
    fn connect_shortcuts_inhibited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcuts_inhibited_trampoline<
            P: IsA<Toplevel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::shortcuts-inhibited".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_shortcuts_inhibited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "startup-id")]
    fn connect_startup_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_startup_id_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::startup-id".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_startup_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state")]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::state".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<Toplevel>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::title".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "transient-for")]
    fn connect_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_transient_for_trampoline<
            P: IsA<Toplevel>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkToplevel,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Toplevel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                c"notify::transient-for".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_transient_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Toplevel>> ToplevelExt for O {}
