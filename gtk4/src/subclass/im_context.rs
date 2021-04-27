// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{IMContext, Widget};
use glib::translate::*;
use glib::{Cast, GString, IsA, Object};
use pango::AttrList;

#[allow(clippy::upper_case_acronyms)]
pub trait IMContextImpl: IMContextImplExt + ObjectImpl {
    fn commit(&self, im_context: &Self::Type, string: &str) {
        self.parent_commit(im_context, string)
    }
    fn delete_surrounding(&self, im_context: &Self::Type, offset: i32, n_chars: i32) -> bool {
        self.parent_delete_surrounding(im_context, offset, n_chars)
    }
    fn filter_keypress(&self, im_context: &Self::Type, event: &gdk::Event) -> bool {
        self.parent_filter_keypress(im_context, event)
    }
    fn focus_in(&self, im_context: &Self::Type) {
        self.parent_focus_in(im_context)
    }
    fn focus_out(&self, im_context: &Self::Type) {
        self.parent_focus_out(im_context)
    }
    fn preedit_string(&self, im_context: &Self::Type) -> (GString, AttrList, i32) {
        self.parent_preedit_string(im_context)
    }
    fn surrounding(&self, im_context: &Self::Type) -> Option<(GString, i32)> {
        self.parent_surrounding(im_context)
    }
    fn preedit_changed(&self, im_context: &Self::Type) {
        self.parent_preedit_changed(im_context)
    }
    fn preedit_end(&self, im_context: &Self::Type) {
        self.parent_preedit_end(im_context)
    }
    fn preedit_start(&self, im_context: &Self::Type) {
        self.parent_preedit_start(im_context)
    }
    fn reset(&self, im_context: &Self::Type) {
        self.parent_reset(im_context)
    }
    fn retrieve_surrounding(&self, im_context: &Self::Type) -> bool {
        self.parent_retrieve_surrounding(im_context)
    }
    fn set_client_widget<P: IsA<Widget>>(&self, im_context: &Self::Type, widget: Option<&P>) {
        self.parent_set_client_widget(im_context, widget)
    }
    fn set_cursor_location(&self, im_context: &Self::Type, area: &gdk::Rectangle) {
        self.parent_set_cursor_location(im_context, area)
    }
    fn set_surrounding(&self, im_context: &Self::Type, text: &str, cursor_index: i32) {
        self.parent_set_surrounding(im_context, text, cursor_index)
    }
    fn set_use_preedit(&self, im_context: &Self::Type, use_preedit: bool) {
        self.parent_set_use_preedit(im_context, use_preedit)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait IMContextImplExt: ObjectSubclass {
    fn parent_commit(&self, im_context: &Self::Type, string: &str);
    fn parent_delete_surrounding(&self, im_context: &Self::Type, offset: i32, n_chars: i32)
        -> bool;
    fn parent_filter_keypress(&self, im_context: &Self::Type, event: &gdk::Event) -> bool;
    fn parent_focus_in(&self, im_context: &Self::Type);
    fn parent_focus_out(&self, im_context: &Self::Type);
    fn parent_preedit_string(&self, im_context: &Self::Type) -> (GString, AttrList, i32);
    fn parent_surrounding(&self, im_context: &Self::Type) -> Option<(GString, i32)>;
    fn parent_preedit_changed(&self, im_context: &Self::Type);
    fn parent_preedit_end(&self, im_context: &Self::Type);
    fn parent_preedit_start(&self, im_context: &Self::Type);
    fn parent_reset(&self, im_context: &Self::Type);
    fn parent_retrieve_surrounding(&self, im_context: &Self::Type) -> bool;
    fn parent_set_client_widget<P: IsA<Widget>>(&self, im_context: &Self::Type, widget: Option<&P>);
    fn parent_set_cursor_location(&self, im_context: &Self::Type, area: &gdk::Rectangle);
    fn parent_set_surrounding(&self, im_context: &Self::Type, text: &str, cursor_index: i32);
    fn parent_set_use_preedit(&self, im_context: &Self::Type, use_preedit: bool);
}

impl<T: IMContextImpl> IMContextImplExt for T {
    fn parent_commit(&self, im_context: &Self::Type, string: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).commit {
                f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    string.to_glib_none().0,
                );
            }
        }
    }

    fn parent_delete_surrounding(
        &self,
        im_context: &Self::Type,
        offset: i32,
        n_chars: i32,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).delete_surrounding {
                from_glib(f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    offset,
                    n_chars,
                ))
            } else {
                // Returns true if the signal was handled
                false
            }
        }
    }

    fn parent_filter_keypress(&self, im_context: &Self::Type, event: &gdk::Event) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).filter_keypress {
                from_glib(f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    event.to_glib_none().0,
                ))
            } else {
                // Returns true if the event was consumed
                false
            }
        }
    }

    fn parent_focus_in(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).focus_in {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_focus_out(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).focus_out {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_surrounding(&self, im_context: &Self::Type) -> Option<(GString, i32)> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).get_surrounding {
                let mut text = std::ptr::null_mut();
                let mut cursor_index = std::mem::MaybeUninit::uninit();
                let ret = from_glib(f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
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

    fn parent_preedit_string(&self, im_context: &Self::Type) -> (GString, AttrList, i32) {
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
                im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
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

    fn parent_preedit_changed(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_changed {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_preedit_end(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_end {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_preedit_start(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).preedit_start {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_reset(&self, im_context: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).reset {
                f(im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0)
            }
        }
    }

    fn parent_retrieve_surrounding(&self, im_context: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).retrieve_surrounding {
                from_glib(f(im_context
                    .unsafe_cast_ref::<IMContext>()
                    .to_glib_none()
                    .0))
            } else {
                // Returns true if the signal was handled
                false
            }
        }
    }

    fn parent_set_client_widget<P: IsA<Widget>>(
        &self,
        im_context: &Self::Type,
        widget: Option<&P>,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_client_widget {
                f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    widget.map(|p| p.as_ref()).to_glib_none().0,
                )
            }
        }
    }

    fn parent_set_cursor_location(&self, im_context: &Self::Type, area: &gdk::Rectangle) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_cursor_location {
                f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    area.to_glib_none().0 as *mut _,
                );
            }
        }
    }

    fn parent_set_surrounding(&self, im_context: &Self::Type, text: &str, cursor_index: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_surrounding {
                f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    text.to_glib_none().0,
                    text.len() as i32,
                    cursor_index,
                )
            }
        }
    }

    fn parent_set_use_preedit(&self, im_context: &Self::Type, use_preedit: bool) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkIMContextClass;
            if let Some(f) = (*parent_class).set_use_preedit {
                f(
                    im_context.unsafe_cast_ref::<IMContext>().to_glib_none().0,
                    use_preedit.into_glib(),
                )
            }
        }
    }
}

unsafe impl<T: IMContextImpl> IsSubclassable<T> for IMContext {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

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
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn im_context_commit<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    stringptr: *const libc::c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);
    let string: Borrowed<GString> = from_glib_borrow(stringptr);

    imp.commit(wrap.unsafe_cast_ref(), string.as_str())
}

unsafe extern "C" fn im_context_delete_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    offset: i32,
    n_chars: i32,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.delete_surrounding(wrap.unsafe_cast_ref(), offset, n_chars)
        .into_glib()
}

unsafe extern "C" fn im_context_filter_keypress<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    eventptr: *mut gdk::ffi::GdkEvent,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);
    let event: Borrowed<gdk::Event> = from_glib_borrow(eventptr);
    imp.filter_keypress(wrap.unsafe_cast_ref(), &event)
        .into_glib()
}

unsafe extern "C" fn im_context_focus_in<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.focus_in(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_focus_out<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.focus_out(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_get_preedit_string<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    text_ptr: *mut *mut libc::c_char,
    attrs_ptr: *mut *mut pango::ffi::PangoAttrList,
    cursor_index_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    let (text, attrs, cursor_idx) = imp.preedit_string(wrap.unsafe_cast_ref());

    *text_ptr = text.to_glib_full();
    *cursor_index_ptr = cursor_idx;
    *attrs_ptr = attrs.to_glib_full();
}

unsafe extern "C" fn im_context_get_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    text_ptr: *mut *mut libc::c_char,
    cursor_index_ptr: *mut libc::c_int,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    if let Some((text, cursor_idx)) = imp.surrounding(wrap.unsafe_cast_ref()) {
        *text_ptr = text.to_glib_full();
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
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.preedit_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_preedit_end<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.preedit_end(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_preedit_start<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.preedit_start(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_reset<T: IMContextImpl>(ptr: *mut ffi::GtkIMContext) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.reset(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn im_context_retrieve_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.retrieve_surrounding(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn im_context_set_client_widget<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    widgetptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);
    let widget: Borrowed<Option<Widget>> = from_glib_borrow(widgetptr);

    imp.set_client_widget(wrap.unsafe_cast_ref(), widget.as_ref().as_ref());
}

unsafe extern "C" fn im_context_set_cursor_location<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    areaptr: *mut gdk::ffi::GdkRectangle,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);
    let area = from_glib_borrow(areaptr);

    imp.set_cursor_location(wrap.unsafe_cast_ref(), &area);
}

unsafe extern "C" fn im_context_set_surrounding<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    textptr: *const libc::c_char,
    length: i32,
    cursor_index: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);
    let text: Borrowed<GString> = from_glib_borrow(textptr);

    // length == -1 if text is null-terminated
    let text = if length == -1 {
        &text[..]
    } else {
        &text[0..(length as usize)]
    };

    imp.set_surrounding(wrap.unsafe_cast_ref(), text, cursor_index)
}

unsafe extern "C" fn im_context_set_use_preedit<T: IMContextImpl>(
    ptr: *mut ffi::GtkIMContext,
    use_preedit: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<IMContext> = from_glib_borrow(ptr);

    imp.set_use_preedit(wrap.unsafe_cast_ref(), from_glib(use_preedit))
}
