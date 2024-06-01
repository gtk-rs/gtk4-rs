// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::{ffi, prelude::*, StyleContext, StyleProvider};

impl StyleContext {
    #[deprecated(note = "Use gtk::style_context_add_provider_for_display instead.")]
    #[doc(alias = "gtk_style_context_add_provider_for_display")]
    pub fn add_provider_for_display(
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

    #[deprecated(note = "Use gtk::style_context_remove_provider_for_display instead.")]
    #[doc(alias = "gtk_style_context_remove_provider_for_display")]
    pub fn remove_provider_for_display(
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
}
