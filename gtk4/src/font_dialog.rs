// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, FontDialog, Window};
use glib::translate::*;
use std::{boxed::Box as Box_, pin::Pin, ptr};

impl FontDialog {
    #[doc(alias = "gtk_font_dialog_choose_font_and_features")]
    #[doc(alias = "gtk_font_dialog_choose_font_and_features_finish")]
    pub fn choose_font_and_features<
        P: FnOnce(
                Result<
                    (
                        Option<pango::FontDescription>,
                        glib::GString,
                        pango::Language,
                    ),
                    glib::Error,
                >,
            ) + 'static,
    >(
        &self,
        parent: Option<&impl IsA<Window>>,
        initial_value: Option<&pango::FontDescription>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
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
        unsafe extern "C" fn choose_font_and_features_trampoline<
            P: FnOnce(
                    Result<
                        (
                            Option<pango::FontDescription>,
                            glib::GString,
                            pango::Language,
                        ),
                        glib::Error,
                    >,
                ) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut font_desc = ptr::null_mut();
            let mut font_features = ptr::null_mut();
            let mut language = ptr::null_mut();
            let _ = ffi::gtk_font_dialog_choose_font_and_features_finish(
                _source_object as *mut _,
                res,
                &mut font_desc,
                &mut font_features,
                &mut language,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    from_glib_full(font_desc),
                    from_glib_full(font_features),
                    from_glib_full(language),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_font_and_features_trampoline::<P>;
        unsafe {
            ffi::gtk_font_dialog_choose_font_and_features(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                mut_override(initial_value.to_glib_none().0),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[allow(clippy::type_complexity)]
    pub fn choose_font_and_features_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
        initial_value: Option<&pango::FontDescription>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<
                    Output = Result<
                        (
                            Option<pango::FontDescription>,
                            glib::GString,
                            pango::Language,
                        ),
                        glib::Error,
                    >,
                > + 'static,
        >,
    > {
        let parent = parent.map(ToOwned::to_owned);
        let initial_value = initial_value.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.choose_font_and_features(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                initial_value.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }
}
