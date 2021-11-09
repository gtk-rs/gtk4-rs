// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Device;
use crate::FullscreenMode;
use crate::Surface;
use crate::SurfaceEdge;
use crate::Texture;
#[cfg(any(feature = "v4_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
use crate::TitlebarGesture;
use crate::ToplevelLayout;
use crate::ToplevelState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

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

pub trait ToplevelExt: 'static {
    #[doc(alias = "gdk_toplevel_begin_move")]
    fn begin_move(&self, device: &impl IsA<Device>, button: i32, x: f64, y: f64, timestamp: u32);

    #[doc(alias = "gdk_toplevel_begin_resize")]
    fn begin_resize(
        &self,
        edge: SurfaceEdge,
        device: Option<&impl IsA<Device>>,
        button: i32,
        x: f64,
        y: f64,
        timestamp: u32,
    );

    #[doc(alias = "gdk_toplevel_focus")]
    fn focus(&self, timestamp: u32);

    #[doc(alias = "gdk_toplevel_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> ToplevelState;

    #[doc(alias = "gdk_toplevel_lower")]
    fn lower(&self) -> bool;

    #[doc(alias = "gdk_toplevel_minimize")]
    fn minimize(&self) -> bool;

    #[doc(alias = "gdk_toplevel_present")]
    fn present(&self, layout: &ToplevelLayout);

    #[doc(alias = "gdk_toplevel_restore_system_shortcuts")]
    fn restore_system_shortcuts(&self);

    #[doc(alias = "gdk_toplevel_set_decorated")]
    fn set_decorated(&self, decorated: bool);

    #[doc(alias = "gdk_toplevel_set_deletable")]
    fn set_deletable(&self, deletable: bool);

    #[doc(alias = "gdk_toplevel_set_icon_list")]
    fn set_icon_list(&self, surfaces: &[Texture]);

    #[doc(alias = "gdk_toplevel_set_modal")]
    fn set_modal(&self, modal: bool);

    #[doc(alias = "gdk_toplevel_set_startup_id")]
    fn set_startup_id(&self, startup_id: &str);

    #[doc(alias = "gdk_toplevel_set_title")]
    fn set_title(&self, title: &str);

    #[doc(alias = "gdk_toplevel_set_transient_for")]
    fn set_transient_for(&self, parent: &impl IsA<Surface>);

    #[doc(alias = "gdk_toplevel_supports_edge_constraints")]
    fn supports_edge_constraints(&self) -> bool;

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_toplevel_titlebar_gesture")]
    fn titlebar_gesture(&self, gesture: TitlebarGesture) -> bool;

    fn is_decorated(&self) -> bool;

    fn is_deletable(&self) -> bool;

    #[doc(alias = "fullscreen-mode")]
    fn fullscreen_mode(&self) -> FullscreenMode;

    #[doc(alias = "fullscreen-mode")]
    fn set_fullscreen_mode(&self, fullscreen_mode: FullscreenMode);

    //#[doc(alias = "icon-list")]
    //fn icon_list(&self) -> /*Unimplemented*/Fundamental: Pointer;

    fn is_modal(&self) -> bool;

    #[doc(alias = "shortcuts-inhibited")]
    fn is_shortcuts_inhibited(&self) -> bool;

    #[doc(alias = "startup-id")]
    fn startup_id(&self) -> Option<glib::GString>;

    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "transient-for")]
    fn transient_for(&self) -> Option<Surface>;

    #[doc(alias = "decorated")]
    fn connect_decorated_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "deletable")]
    fn connect_deletable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "fullscreen-mode")]
    fn connect_fullscreen_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-list")]
    fn connect_icon_list_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "modal")]
    fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "shortcuts-inhibited")]
    fn connect_shortcuts_inhibited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "startup-id")]
    fn connect_startup_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "state")]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "transient-for")]
    fn connect_transient_for_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Toplevel>> ToplevelExt for O {
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

    fn focus(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_toplevel_focus(self.as_ref().to_glib_none().0, timestamp);
        }
    }

    fn state(&self) -> ToplevelState {
        unsafe { from_glib(ffi::gdk_toplevel_get_state(self.as_ref().to_glib_none().0)) }
    }

    fn lower(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_lower(self.as_ref().to_glib_none().0)) }
    }

    fn minimize(&self) -> bool {
        unsafe { from_glib(ffi::gdk_toplevel_minimize(self.as_ref().to_glib_none().0)) }
    }

    fn present(&self, layout: &ToplevelLayout) {
        unsafe {
            ffi::gdk_toplevel_present(self.as_ref().to_glib_none().0, layout.to_glib_none().0);
        }
    }

    fn restore_system_shortcuts(&self) {
        unsafe {
            ffi::gdk_toplevel_restore_system_shortcuts(self.as_ref().to_glib_none().0);
        }
    }

    fn set_decorated(&self, decorated: bool) {
        unsafe {
            ffi::gdk_toplevel_set_decorated(self.as_ref().to_glib_none().0, decorated.into_glib());
        }
    }

    fn set_deletable(&self, deletable: bool) {
        unsafe {
            ffi::gdk_toplevel_set_deletable(self.as_ref().to_glib_none().0, deletable.into_glib());
        }
    }

    fn set_icon_list(&self, surfaces: &[Texture]) {
        unsafe {
            ffi::gdk_toplevel_set_icon_list(
                self.as_ref().to_glib_none().0,
                surfaces.to_glib_none().0,
            );
        }
    }

    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gdk_toplevel_set_modal(self.as_ref().to_glib_none().0, modal.into_glib());
        }
    }

    fn set_startup_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_toplevel_set_startup_id(
                self.as_ref().to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gdk_toplevel_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_transient_for(&self, parent: &impl IsA<Surface>) {
        unsafe {
            ffi::gdk_toplevel_set_transient_for(
                self.as_ref().to_glib_none().0,
                parent.as_ref().to_glib_none().0,
            );
        }
    }

    fn supports_edge_constraints(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_supports_edge_constraints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    fn titlebar_gesture(&self, gesture: TitlebarGesture) -> bool {
        unsafe {
            from_glib(ffi::gdk_toplevel_titlebar_gesture(
                self.as_ref().to_glib_none().0,
                gesture.into_glib(),
            ))
        }
    }

    fn is_decorated(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "decorated")
    }

    fn is_deletable(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "deletable")
    }

    fn fullscreen_mode(&self) -> FullscreenMode {
        glib::ObjectExt::property(self.as_ref(), "fullscreen-mode")
    }

    fn set_fullscreen_mode(&self, fullscreen_mode: FullscreenMode) {
        glib::ObjectExt::set_property(self.as_ref(), "fullscreen-mode", &fullscreen_mode)
    }

    //fn icon_list(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    glib::ObjectExt::property(self.as_ref(), "icon-list")
    //}

    fn is_modal(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "modal")
    }

    fn is_shortcuts_inhibited(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "shortcuts-inhibited")
    }

    fn startup_id(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "startup-id")
    }

    fn title(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "title")
    }

    fn transient_for(&self) -> Option<Surface> {
        glib::ObjectExt::property(self.as_ref(), "transient-for")
    }

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
                b"notify::decorated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_decorated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::deletable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_deletable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::fullscreen-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fullscreen_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::icon-list\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_list_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::shortcuts-inhibited\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcuts_inhibited_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::startup-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_startup_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

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
                b"notify::transient-for\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transient_for_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Toplevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Toplevel")
    }
}