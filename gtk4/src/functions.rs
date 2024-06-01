// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, pin::Pin, sync::OnceLock};

use glib::{translate::*, Quark, Slice};

pub use crate::auto::functions::*;
use crate::{ffi, prelude::*, AboutDialog, StyleProvider, Window};

#[doc(alias = "gtk_accelerator_valid")]
pub fn accelerator_valid(keyval: gdk::Key, modifiers: gdk::ModifierType) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gtk_accelerator_valid(
            keyval.into_glib(),
            modifiers.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_get_label")]
pub fn accelerator_get_label(
    accelerator_key: gdk::Key,
    accelerator_mods: gdk::ModifierType,
) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label(
            accelerator_key.into_glib(),
            accelerator_mods.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_get_label_with_keycode")]
pub fn accelerator_get_label_with_keycode(
    display: Option<&impl IsA<gdk::Display>>,
    accelerator_key: gdk::Key,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_get_label_with_keycode(
            display.map(|p| p.as_ref()).to_glib_none().0,
            accelerator_key.into_glib(),
            keycode,
            accelerator_mods.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_name")]
pub fn accelerator_name(
    accelerator_key: gdk::Key,
    accelerator_mods: gdk::ModifierType,
) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name(
            accelerator_key.into_glib(),
            accelerator_mods.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_name_with_keycode")]
pub fn accelerator_name_with_keycode(
    display: Option<&impl IsA<gdk::Display>>,
    accelerator_key: gdk::Key,
    keycode: u32,
    accelerator_mods: gdk::ModifierType,
) -> glib::GString {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gtk_accelerator_name_with_keycode(
            display.map(|p| p.as_ref()).to_glib_none().0,
            accelerator_key.into_glib(),
            keycode,
            accelerator_mods.into_glib(),
        ))
    }
}

#[doc(alias = "gtk_accelerator_parse")]
pub fn accelerator_parse(accelerator: impl IntoGStr) -> Option<(gdk::Key, gdk::ModifierType)> {
    assert_initialized_main_thread!();
    unsafe {
        accelerator.run_with_gstr(|accelerator| {
            let mut accelerator_key = std::mem::MaybeUninit::uninit();
            let mut accelerator_mods = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_accelerator_parse(
                accelerator.as_ptr(),
                accelerator_key.as_mut_ptr(),
                accelerator_mods.as_mut_ptr(),
            ));
            if ret {
                Some((
                    gdk::Key::from_glib(accelerator_key.assume_init()),
                    from_glib(accelerator_mods.assume_init()),
                ))
            } else {
                None
            }
        })
    }
}

#[doc(alias = "gtk_accelerator_parse_with_keycode")]
pub fn accelerator_parse_with_keycode(
    accelerator: impl IntoGStr,
    display: Option<&impl IsA<gdk::Display>>,
) -> Option<(gdk::Key, Slice<u32>, gdk::ModifierType)> {
    assert_initialized_main_thread!();
    unsafe {
        accelerator.run_with_gstr(|accelerator| {
            let mut accelerator_key = std::mem::MaybeUninit::uninit();
            let mut accelerator_codes_ptr = std::ptr::null_mut();
            let mut accelerator_mods = std::mem::MaybeUninit::uninit();
            let success = from_glib(ffi::gtk_accelerator_parse_with_keycode(
                accelerator.as_ptr(),
                display.map(|p| p.as_ref()).to_glib_none().0,
                accelerator_key.as_mut_ptr(),
                &mut accelerator_codes_ptr,
                accelerator_mods.as_mut_ptr(),
            ));
            if success {
                let mut len = 0;
                if !accelerator_codes_ptr.is_null() {
                    while std::ptr::read(accelerator_codes_ptr.add(len)) != 0 {
                        len += 1;
                    }
                }
                let accelerator_codes = Slice::from_glib_full_num(accelerator_codes_ptr, len);
                Some((
                    gdk::Key::from_glib(accelerator_key.assume_init()),
                    accelerator_codes,
                    from_glib(accelerator_mods.assume_init()),
                ))
            } else {
                None
            }
        })
    }
}

#[doc(alias = "gtk_show_uri_full")]
#[doc(alias = "gtk_show_uri_full_finish")]
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub fn show_uri_full<P: FnOnce(Result<(), glib::Error>) + 'static>(
    parent: Option<&impl IsA<Window>>,
    uri: &str,
    timestamp: u32,
    cancellable: Option<&impl IsA<gio::Cancellable>>,
    callback: P,
) {
    assert_initialized_main_thread!();
    let main_context = glib::MainContext::ref_thread_default();
    let is_main_context_owner = main_context.is_owner();
    let has_acquired_main_context = (!is_main_context_owner)
        .then(|| main_context.acquire().ok())
        .flatten();
    assert!(
        is_main_context_owner || has_acquired_main_context.is_some(),
        "Async operations only allowed if the thread is owning the MainContext"
    );

    let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
        Box_::new(glib::thread_guard::ThreadGuard::new(callback));
    unsafe extern "C" fn show_uri_full_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
        parent_ptr: *mut glib::gobject_ffi::GObject,
        res: *mut gio::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = std::ptr::null_mut();
        let _ = ffi::gtk_show_uri_full_finish(parent_ptr as *mut ffi::GtkWindow, res, &mut error);
        let result = if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        };
        let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::from_raw(user_data as *mut _);
        let callback = callback.into_inner();
        callback(result);
    }
    let callback = show_uri_full_trampoline::<P>;
    unsafe {
        ffi::gtk_show_uri_full(
            parent.map(|p| p.as_ref()).to_glib_none().0,
            uri.to_glib_none().0,
            timestamp,
            cancellable.map(|p| p.as_ref()).to_glib_none().0,
            Some(callback),
            Box_::into_raw(user_data) as *mut _,
        );
    }
}

#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub fn show_uri_full_future(
    parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    uri: &str,
    timestamp: u32,
) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
    skip_assert_initialized!();
    let parent = parent.map(ToOwned::to_owned);
    let uri = String::from(uri);
    Box_::pin(gio::GioFuture::new(&(), move |_obj, cancellable, send| {
        show_uri_full(
            parent.as_ref().map(::std::borrow::Borrow::borrow),
            &uri,
            timestamp,
            Some(cancellable),
            move |res| {
                send.resolve(res);
            },
        );
    }))
}

#[doc(alias = "gtk_show_about_dialog")]
pub fn show_about_dialog<P: IsA<Window>>(parent: Option<&P>, properties: &[(&str, &dyn ToValue)]) {
    assert_initialized_main_thread!();
    static QUARK: OnceLock<Quark> = OnceLock::new();
    let quark = *QUARK.get_or_init(|| Quark::from_str("gtk-rs-about-dialog"));

    unsafe {
        if let Some(d) = parent.and_then(|p| p.qdata::<AboutDialog>(quark)) {
            d.as_ref().show();
        } else {
            let mut builder = glib::Object::builder::<AboutDialog>();
            for (key, value) in properties {
                builder = builder.property(key, *value);
            }
            let about_dialog = builder.build();
            about_dialog.set_hide_on_close(true);

            // cache the dialog if a parent is set
            if let Some(dialog_parent) = parent {
                about_dialog.set_modal(true);
                about_dialog.set_transient_for(parent);
                about_dialog.set_destroy_with_parent(true);
                dialog_parent.set_qdata(quark, about_dialog.clone());
            }

            about_dialog.show();
        };
    }
}

#[doc(alias = "gtk_test_list_all_types")]
pub fn test_list_all_types() -> Slice<glib::Type> {
    unsafe {
        let mut n_types = std::mem::MaybeUninit::uninit();
        let types = ffi::gtk_test_list_all_types(n_types.as_mut_ptr());
        Slice::from_glib_container_num(types as *mut _, n_types.assume_init() as usize)
    }
}

#[doc(alias = "gtk_style_context_add_provider_for_display")]
#[doc(alias = "add_provider_for_display")]
pub fn style_context_add_provider_for_display(
    display: &impl IsA<gdk::Display>,
    provider: &impl IsA<StyleProvider>,
    priority: u32,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_style_context_add_provider_for_display(
            display.as_ref().to_glib_none().0,
            provider.as_ref().to_glib_none().0,
            priority,
        );
    }
}

#[doc(alias = "gtk_style_context_remove_provider_for_display")]
#[doc(alias = "remove_provider_for_display")]
pub fn style_context_remove_provider_for_display(
    display: &impl IsA<gdk::Display>,
    provider: &impl IsA<StyleProvider>,
) {
    skip_assert_initialized!();
    unsafe {
        ffi::gtk_style_context_remove_provider_for_display(
            display.as_ref().to_glib_none().0,
            provider.as_ref().to_glib_none().0,
        );
    }
}
