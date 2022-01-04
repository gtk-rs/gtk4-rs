// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::InputHints;
use crate::InputPurpose;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GtkIMContext")]
    pub struct IMContext(Object<ffi::GtkIMContext, ffi::GtkIMContextClass>);

    match fn {
        type_ => || ffi::gtk_im_context_get_type(),
    }
}

impl IMContext {
    pub const NONE: Option<&'static IMContext> = None;
}

pub trait IMContextExt: 'static {
    #[doc(alias = "gtk_im_context_delete_surrounding")]
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool;

    #[doc(alias = "gtk_im_context_filter_key")]
    fn filter_key(
        &self,
        press: bool,
        surface: &impl IsA<gdk::Surface>,
        device: &gdk::Device,
        time: u32,
        keycode: u32,
        state: gdk::ModifierType,
        group: i32,
    ) -> bool;

    #[doc(alias = "gtk_im_context_focus_in")]
    fn focus_in(&self);

    #[doc(alias = "gtk_im_context_focus_out")]
    fn focus_out(&self);

    #[doc(alias = "gtk_im_context_get_preedit_string")]
    #[doc(alias = "get_preedit_string")]
    fn preedit_string(&self) -> (glib::GString, pango::AttrList, i32);

    #[cfg_attr(feature = "v4_2", deprecated = "Since 4.2")]
    #[doc(alias = "gtk_im_context_get_surrounding")]
    #[doc(alias = "get_surrounding")]
    fn surrounding(&self) -> Option<(glib::GString, i32)>;

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gtk_im_context_get_surrounding_with_selection")]
    #[doc(alias = "get_surrounding_with_selection")]
    fn surrounding_with_selection(&self) -> Option<(glib::GString, i32, i32)>;

    #[doc(alias = "gtk_im_context_reset")]
    fn reset(&self);

    #[doc(alias = "gtk_im_context_set_client_widget")]
    fn set_client_widget(&self, widget: Option<&impl IsA<Widget>>);

    #[doc(alias = "gtk_im_context_set_cursor_location")]
    fn set_cursor_location(&self, area: &gdk::Rectangle);

    #[cfg_attr(feature = "v4_2", deprecated = "Since 4.2")]
    #[doc(alias = "gtk_im_context_set_surrounding")]
    fn set_surrounding(&self, text: &str, cursor_index: i32);

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    #[doc(alias = "gtk_im_context_set_surrounding_with_selection")]
    fn set_surrounding_with_selection(&self, text: &str, cursor_index: i32, anchor_index: i32);

    #[doc(alias = "gtk_im_context_set_use_preedit")]
    fn set_use_preedit(&self, use_preedit: bool);

    #[doc(alias = "input-hints")]
    fn input_hints(&self) -> InputHints;

    #[doc(alias = "input-hints")]
    fn set_input_hints(&self, input_hints: InputHints);

    #[doc(alias = "input-purpose")]
    fn input_purpose(&self) -> InputPurpose;

    #[doc(alias = "input-purpose")]
    fn set_input_purpose(&self, input_purpose: InputPurpose);

    #[doc(alias = "commit")]
    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "delete-surrounding")]
    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "preedit-changed")]
    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "preedit-end")]
    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "preedit-start")]
    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "retrieve-surrounding")]
    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[doc(alias = "input-hints")]
    fn connect_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "input-purpose")]
    fn connect_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<IMContext>> IMContextExt for O {
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool {
        unsafe {
            from_glib(ffi::gtk_im_context_delete_surrounding(
                self.as_ref().to_glib_none().0,
                offset,
                n_chars,
            ))
        }
    }

    fn filter_key(
        &self,
        press: bool,
        surface: &impl IsA<gdk::Surface>,
        device: &gdk::Device,
        time: u32,
        keycode: u32,
        state: gdk::ModifierType,
        group: i32,
    ) -> bool {
        unsafe {
            from_glib(ffi::gtk_im_context_filter_key(
                self.as_ref().to_glib_none().0,
                press.into_glib(),
                surface.as_ref().to_glib_none().0,
                device.to_glib_none().0,
                time,
                keycode,
                state.into_glib(),
                group,
            ))
        }
    }

    fn focus_in(&self) {
        unsafe {
            ffi::gtk_im_context_focus_in(self.as_ref().to_glib_none().0);
        }
    }

    fn focus_out(&self) {
        unsafe {
            ffi::gtk_im_context_focus_out(self.as_ref().to_glib_none().0);
        }
    }

    fn preedit_string(&self) -> (glib::GString, pango::AttrList, i32) {
        unsafe {
            let mut str = ptr::null_mut();
            let mut attrs = ptr::null_mut();
            let mut cursor_pos = mem::MaybeUninit::uninit();
            ffi::gtk_im_context_get_preedit_string(
                self.as_ref().to_glib_none().0,
                &mut str,
                &mut attrs,
                cursor_pos.as_mut_ptr(),
            );
            let cursor_pos = cursor_pos.assume_init();
            (from_glib_full(str), from_glib_full(attrs), cursor_pos)
        }
    }

    fn surrounding(&self) -> Option<(glib::GString, i32)> {
        unsafe {
            let mut text = ptr::null_mut();
            let mut cursor_index = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_im_context_get_surrounding(
                self.as_ref().to_glib_none().0,
                &mut text,
                cursor_index.as_mut_ptr(),
            ));
            let cursor_index = cursor_index.assume_init();
            if ret {
                Some((from_glib_full(text), cursor_index))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    fn surrounding_with_selection(&self) -> Option<(glib::GString, i32, i32)> {
        unsafe {
            let mut text = ptr::null_mut();
            let mut cursor_index = mem::MaybeUninit::uninit();
            let mut anchor_index = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_im_context_get_surrounding_with_selection(
                self.as_ref().to_glib_none().0,
                &mut text,
                cursor_index.as_mut_ptr(),
                anchor_index.as_mut_ptr(),
            ));
            let cursor_index = cursor_index.assume_init();
            let anchor_index = anchor_index.assume_init();
            if ret {
                Some((from_glib_full(text), cursor_index, anchor_index))
            } else {
                None
            }
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_im_context_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_client_widget(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_im_context_set_client_widget(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_cursor_location(&self, area: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_im_context_set_cursor_location(
                self.as_ref().to_glib_none().0,
                area.to_glib_none().0,
            );
        }
    }

    fn set_surrounding(&self, text: &str, cursor_index: i32) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_im_context_set_surrounding(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                len,
                cursor_index,
            );
        }
    }

    #[cfg(any(feature = "v4_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_2")))]
    fn set_surrounding_with_selection(&self, text: &str, cursor_index: i32, anchor_index: i32) {
        let len = text.len() as i32;
        unsafe {
            ffi::gtk_im_context_set_surrounding_with_selection(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
                len,
                cursor_index,
                anchor_index,
            );
        }
    }

    fn set_use_preedit(&self, use_preedit: bool) {
        unsafe {
            ffi::gtk_im_context_set_use_preedit(
                self.as_ref().to_glib_none().0,
                use_preedit.into_glib(),
            );
        }
    }

    fn input_hints(&self) -> InputHints {
        glib::ObjectExt::property(self.as_ref(), "input-hints")
    }

    fn set_input_hints(&self, input_hints: InputHints) {
        glib::ObjectExt::set_property(self.as_ref(), "input-hints", &input_hints)
    }

    fn input_purpose(&self) -> InputPurpose {
        glib::ObjectExt::property(self.as_ref(), "input-purpose")
    }

    fn set_input_purpose(&self, input_purpose: InputPurpose) {
        glib::ObjectExt::set_property(self.as_ref(), "input-purpose", &input_purpose)
    }

    fn connect_commit<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn commit_trampoline<P: IsA<IMContext>, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GtkIMContext,
            str: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                IMContext::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(str),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"commit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    commit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_delete_surrounding<F: Fn(&Self, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn delete_surrounding_trampoline<
            P: IsA<IMContext>,
            F: Fn(&P, i32, i32) -> bool + 'static,
        >(
            this: *mut ffi::GtkIMContext,
            offset: libc::c_int,
            n_chars: libc::c_int,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                IMContext::from_glib_borrow(this).unsafe_cast_ref(),
                offset,
                n_chars,
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"delete-surrounding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    delete_surrounding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_preedit_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_changed_trampoline<P: IsA<IMContext>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkIMContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_preedit_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_end_trampoline<P: IsA<IMContext>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkIMContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_preedit_start<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn preedit_start_trampoline<P: IsA<IMContext>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkIMContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preedit-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    preedit_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_retrieve_surrounding<F: Fn(&Self) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn retrieve_surrounding_trampoline<
            P: IsA<IMContext>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkIMContext,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"retrieve-surrounding\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    retrieve_surrounding_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_input_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_hints_trampoline<
            P: IsA<IMContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkIMContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-hints\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_hints_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_input_purpose_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_input_purpose_trampoline<
            P: IsA<IMContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkIMContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(IMContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::input-purpose\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_input_purpose_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for IMContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("IMContext")
    }
}