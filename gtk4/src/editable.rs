use glib::object::Cast;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;
use glib::IsA;
use gtk_sys::GtkEditable;
use libc::{c_char, c_int, c_uchar};
use std::ffi::CStr;
use std::mem::transmute;
use std::slice;
use std::str;
use Editable;

impl Editable {
    pub fn delegate_get_property<P: IsA<Editable> + IsA<glib::Object>>(
        object: &P,
        prop_id: u32,
        value: &mut glib::Value,
        pspec: &glib::ParamSpec,
    ) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(gtk_sys::gtk_editable_delegate_get_property(
                object.upcast_ref::<glib::Object>().to_glib_none().0,
                prop_id,
                value.to_glib_none_mut().0,
                pspec.to_glib_none().0,
            ))
        }
    }

    pub fn delegate_set_property<P: IsA<Editable> + IsA<glib::Object>>(
        object: &P,
        prop_id: u32,
        value: &glib::Value,
        pspec: &glib::ParamSpec,
    ) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(gtk_sys::gtk_editable_delegate_set_property(
                object.upcast_ref::<glib::Object>().to_glib_none().0,
                prop_id,
                value.to_glib_none().0,
                pspec.to_glib_none().0,
            ))
        }
    }
}

pub trait EditableExtManual: 'static {
    fn connect_insert_text<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &str, &mut i32) + 'static;
}

impl<T: IsA<Editable>> EditableExtManual for T {
    fn connect_insert_text<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &str, &mut i32) + 'static,
    {
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.to_glib_none().0 as *mut _,
                b"insert-text\0".as_ptr() as *mut _,
                Some(transmute(insert_text_trampoline::<Self, F> as usize)),
                Box::into_raw(f),
            )
        }
    }
}

unsafe extern "C" fn insert_text_trampoline<T, F: Fn(&T, &str, &mut i32) + 'static>(
    this: *mut GtkEditable,
    new_text: *mut c_char,
    new_text_length: c_int,
    position: *mut c_int,
    f: &F,
) where
    T: IsA<Editable>,
{
    let buf = if new_text_length != -1 {
        slice::from_raw_parts(new_text as *mut c_uchar, new_text_length as usize)
    } else {
        CStr::from_ptr(new_text).to_bytes()
    };
    let string = str::from_utf8(buf).unwrap();
    f(
        &Editable::from_glib_borrow(this).unsafe_cast_ref(),
        string,
        transmute(position),
    );
}
