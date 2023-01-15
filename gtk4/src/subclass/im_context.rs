// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`IMContext`](crate::IMContext).

use crate::{prelude::*, subclass::prelude::*, IMContext, Widget};
use glib::{translate::*, GString};
use pango::AttrList;

#[allow(clippy::upper_case_acronyms)]
pub trait IMContextImpl: IMContextImplExt + ObjectImpl {
    fn commit(&self, string: &str) {
        self.parent_commit(string)
    }
    fn delete_surrounding(&self, offset: i32, n_chars: i32) -> bool {
        self.parent_delete_surrounding(offset, n_chars)
    }
    fn filter_keypress(&self, event: &gdk::Event) -> bool {
        self.parent_filter_keypress(event)
    }
    fn focus_in(&self) {
        self.parent_focus_in()
    }
    fn focus_out(&self) {
        self.parent_focus_out()
    }
    #[doc(alias = "get_preedit_string")]
    fn preedit_string(&self) -> (GString, AttrList, i32) {
        self.parent_preedit_string()
    }
    #[doc(alias = "get_surrounding")]
    fn surrounding(&self) -> Option<(GString, i32)> {
        self.parent_surrounding()
    }
    fn preedit_changed(&self) {
        self.parent_preedit_changed()
    }
    fn preedit_end(&self) {
        self.parent_preedit_end()
    }
    fn preedit_start(&self) {
        self.parent_preedit_start()
    }
    fn reset(&self) {
        self.parent_reset()
    }
    fn retrieve_surrounding(&self) -> bool {
        self.parent_retrieve_surrounding()
    }
    fn set_client_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        self.parent_set_client_widget(widget)
    }
    fn set_cursor_location(&self, area: &gdk::Rectangle) {
        self.parent_set_cursor_location(area)
    }
    #[cfg_attr(feature = "v4_2", deprecated = "Since 4.2")]
    #[allow(deprecated)]
    fn set_surrounding(&self, text: &str, cursor_index: i32) {
        self.parent_set_surrounding(text, cursor_index)
    }
    fn set_use_preedit(&self, use_preedit: bool) {
        self.parent_set_use_preedit(use_preedit)
    }
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    fn activate_osk(&self) {
        self.parent_activate_osk()
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait IMContextImplExt: ObjectSubclass {
    fn parent_commit(&self, string: &str);
    fn parent_delete_surrounding(&self, offset: i32, n_chars: i32) -> bool;
    fn parent_filter_keypress(&self, event: &gdk::Event) -> bool;
    fn parent_focus_in(&self);
    fn parent_focus_out(&self);
    fn parent_preedit_string(&self) -> (GString, AttrList, i32);
    fn parent_surrounding(&self) -> Option<(GString, i32)>;
    fn parent_preedit_changed(&self);
    fn parent_preedit_end(&self);
    fn parent_preedit_start(&self);
    fn parent_reset(&self);
    fn parent_retrieve_surrounding(&self) -> bool;
    fn parent_set_client_widget<P: IsA<Widget>>(&self, widget: Option<&P>);
    fn parent_set_cursor_location(&self, area: &gdk::Rectangle);
    #[cfg_attr(feature = "v4_2", deprecated = "Since 4.2")]
    #[allow(deprecated)]
    fn parent_set_surrounding(&self, text: &str, cursor_index: i32);
    fn parent_set_use_preedit(&self, use_preedit: bool);
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    fn parent_activate_osk(&self);
}

impl<T: IMContextImpl> IMContextImplExt for T {
    fn parent_commit(&self, string: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).commit {
                f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    string.to_glib_none().0,
                );
            }
        }
    }

    fn parent_delete_surrounding(&self, offset: i32, n_chars: i32) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).delete_surrounding {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    offset,
                    n_chars,
                ))
            } else {
                // Returns true if the signal was handled
                false
            }
        }
    }

    fn parent_filter_keypress(&self, event: &gdk::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).filter_keypress {
                from_glib(f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    event.to_glib_none().0,
                ))
            } else {
                // Returns true if the event was consumed
                false
            }
        }
    }

    fn parent_focus_in(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).focus_in {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_focus_out(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).focus_out {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_surrounding(&self) -> Option<(GString, i32)> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).get_surrounding {
                let mut text = std::ptr::null_mut();
                let mut cursor_index = std::mem::MaybeUninit::uninit();
                let ret = from_glib(f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    &mut text,
                    cursor_index.as_mut_ptr(),
                ));
                if ret {
                    return Some((from_glib_full(text), cursor_index.assume_init()));
                }
            }
            None
        }
    }

    fn parent_preedit_string(&self) -> (GString, AttrList, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            let f = (*parent_class)
                .get_preedit_string
                .expect("No parent class impl for \"get_preedit_string\"");
            let mut string = std::ptr::null_mut();
            let mut attrs = std::ptr::null_mut();
            let mut cursor_pos = std::mem::MaybeUninit::uninit();
            f(
                self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                &mut string,
                &mut attrs,
                cursor_pos.as_mut_ptr(),
            );
            (
                from_glib_full(string),
                from_glib_full(attrs),
                cursor_pos.assume_init(),
            )
        }
    }

    fn parent_preedit_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_changed {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_preedit_end(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_end {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_preedit_start(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_start {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_reset(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).reset {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_retrieve_surrounding(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).retrieve_surrounding {
                from_glib(f(self
                    .obj()
                    .unsafe_cast_ref::<IMContext>()
                    .to_glib_none()
                    .0))
            } else {
                // Returns true if the signal was handled
                false
            }
        }
    }

    fn parent_set_client_widget<P: IsA<Widget>>(&self, widget: Option<&P>) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_client_widget {
                f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    widget.map(|p| p.as_ref()).to_glib_none().0,
                )
            }
        }
    }

    fn parent_set_cursor_location(&self, area: &gdk::Rectangle) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_cursor_location {
                f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    area.to_glib_none().0 as *mut _,
                );
            }
        }
    }

    fn parent_set_surrounding(&self, text: &str, cursor_index: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_surrounding {
                f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    text.to_glib_none().0,
                    text.len() as i32,
                    cursor_index,
                )
            }
        }
    }

    fn parent_set_use_preedit(&self, use_preedit: bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_use_preedit {
                f(
                    self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    use_preedit.into_glib(),
                )
            }
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    fn parent_activate_osk(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).activate_osk {
                f(self.obj().unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: IMContextImpl> IsSubclassable<T> for IMContext {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.commit = Some(im_context_commit::<T>);
        klass.delete_surrounding = Some(im_context_delete_surrounding::<T>);
        klass.filter_keypress = Some(im_context_filter_keypress::<T>);
        klass.focus_in = Some(im_context_focus_in::<T>);
        klass.focus_out = Some(im_context_focus_out::<T>);
        klass.get_preedit_string = Some(im_context_get_preedit_string::<T>);
        klass.get_surrounding = Some(im_context_get_surrounding::<T>);
        klass.preedit_changed = Some(im_context_preedit_changed::<T>);
        klass.preedit_end = Some(im_context_preedit_end::<T>);
        klass.preedit_start = Some(im_context_preedit_start::<T>);
        klass.reset = Some(im_context_reset::<T>);
        klass.retrieve_surrounding = Some(im_context_retrieve_surrounding::<T>);
        klass.set_client_widget = Some(im_context_set_client_widget::<T>);
        klass.set_cursor_location = Some(im_context_set_cursor_location::<T>);
        klass.set_surrounding = Some(im_context_set_surrounding::<T>);
        klass.set_use_preedit = Some(im_context_set_use_preedit::<T>);
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        {
            klass.activate_osk = Some(im_context_activate_osk::<T>);
        };
    }
}

unsafe extern "C" fn im_context_commit<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    stringptr: *const libc::c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let string: Borrowed<GString> = from_glib_borrow(stringptr);

    imp.commit(string.as_str())
}

unsafe extern "C" fn im_context_delete_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    offset: i32,
    n_chars: i32,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.delete_surrounding(offset, n_chars).into_glib()
}

unsafe extern "C" fn im_context_filter_keypress<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    eventptr: *mut gdk::ffi::GdkEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let event: Borrowed<gdk::Event> = from_glib_borrow(eventptr);
    imp.filter_keypress(&event).into_glib()
}

unsafe extern "C" fn im_context_focus_in<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.focus_in()
}

unsafe extern "C" fn im_context_focus_out<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.focus_out()
}

unsafe extern "C" fn im_context_get_preedit_string<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    text_ptr: *mut *mut libc::c_char,
    attrs_ptr: *mut *mut pango::ffi::PangoAttrList,
    cursor_index_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let (text, attrs, cursor_idx) = imp.preedit_string();

    *text_ptr = text.into_glib_ptr();
    *cursor_index_ptr = cursor_idx;
    *attrs_ptr = attrs.into_glib_ptr();
}

unsafe extern "C" fn im_context_get_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    text_ptr: *mut *mut libc::c_char,
    cursor_index_ptr: *mut libc::c_int,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    if let Some((text, cursor_idx)) = imp.surrounding() {
        *text_ptr = text.into_glib_ptr();
        *cursor_index_ptr = cursor_idx;
        true.into_glib()
    } else {
        *text_ptr = std::ptr::null_mut();
        *cursor_index_ptr = 0;
        false.into_glib()
    }
}

unsafe extern "C" fn im_context_preedit_changed<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.preedit_changed()
}

unsafe extern "C" fn im_context_preedit_end<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.preedit_end()
}

unsafe extern "C" fn im_context_preedit_start<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.preedit_start()
}

unsafe extern "C" fn im_context_reset<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.reset()
}

unsafe extern "C" fn im_context_retrieve_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.retrieve_surrounding().into_glib()
}

unsafe extern "C" fn im_context_set_client_widget<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    widgetptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Option<Widget>> = from_glib_borrow(widgetptr);

    imp.set_client_widget(widget.as_ref().as_ref());
}

unsafe extern "C" fn im_context_set_cursor_location<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    areaptr: *mut gdk::ffi::GdkRectangle,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let area = from_glib_borrow(areaptr);

    imp.set_cursor_location(&area);
}

unsafe extern "C" fn im_context_set_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    textptr: *const libc::c_char,
    length: i32,
    cursor_index: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let text: Borrowed<GString> = from_glib_borrow(textptr);

    // length == -1 if text is null-terminated
    let text = if length == -1 {
        &text[..]
    } else {
        &text[0..(length as usize)]
    };

    imp.set_surrounding(text, cursor_index)
}

unsafe extern "C" fn im_context_set_use_preedit<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    use_preedit: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.set_use_preedit(from_glib(use_preedit))
}

#[cfg(any(feature = "v4_10", feature = "dox"))]
unsafe extern "C" fn im_context_activate_osk<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.activate_osk()
}
