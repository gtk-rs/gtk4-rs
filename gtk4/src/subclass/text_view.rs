// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`TextView`](crate::TextView).

use crate::subclass::prelude::*;
use crate::{
    DeleteType, MovementStep, Snapshot, TextExtendSelection, TextIter, TextView, TextViewLayer,
};
use glib::translate::*;
use glib::Cast;

pub trait TextViewImpl: TextViewImplExt + WidgetImpl {
    fn backspace(&self, text_view: &Self::Type) {
        self.parent_backspace(text_view)
    }

    fn copy_clipboard(&self, text_view: &Self::Type) {
        self.parent_copy_clipboard(text_view)
    }

    fn cut_clipboard(&self, text_view: &Self::Type) {
        self.parent_cut_clipboard(text_view)
    }

    fn delete_from_cursor(&self, text_view: &Self::Type, type_: DeleteType, count: i32) {
        self.parent_delete_from_cursor(text_view, type_, count)
    }

    fn extend_selection(
        &self,
        text_view: &Self::Type,
        granularity: TextExtendSelection,
        location: &TextIter,
        start: &mut TextIter,
        end: &mut TextIter,
    ) -> glib::signal::Inhibit {
        self.parent_extend_selection(text_view, granularity, location, start, end)
    }

    fn insert_at_cursor(&self, text_view: &Self::Type, text: &str) {
        self.parent_insert_at_cursor(text_view, text)
    }

    fn insert_emoji(&self, text_view: &Self::Type) {
        self.parent_insert_emoji(text_view)
    }

    fn move_cursor(
        &self,
        text_view: &Self::Type,
        step: MovementStep,
        count: i32,
        extend_selection: bool,
    ) {
        self.parent_move_cursor(text_view, step, count, extend_selection)
    }

    fn paste_clipboard(&self, text_view: &Self::Type) {
        self.parent_paste_clipboard(text_view)
    }

    fn set_anchor(&self, text_view: &Self::Type) {
        self.parent_set_anchor(text_view)
    }

    fn snapshot_layer(&self, text_view: &Self::Type, layer: TextViewLayer, snapshot: Snapshot) {
        self.parent_snapshot_layer(text_view, layer, snapshot)
    }

    fn toggle_overwrite(&self, text_view: &Self::Type) {
        self.parent_toggle_overwrite(text_view)
    }
}

pub trait TextViewImplExt: ObjectSubclass {
    fn parent_backspace(&self, text_view: &Self::Type);
    fn parent_copy_clipboard(&self, text_view: &Self::Type);
    fn parent_cut_clipboard(&self, text_view: &Self::Type);
    fn parent_delete_from_cursor(&self, text_view: &Self::Type, type_: DeleteType, count: i32);
    fn parent_extend_selection(
        &self,
        text_view: &Self::Type,
        granularity: TextExtendSelection,
        location: &TextIter,
        start: &mut TextIter,
        end: &mut TextIter,
    ) -> glib::signal::Inhibit;
    fn parent_insert_at_cursor(&self, text_view: &Self::Type, text: &str);
    fn parent_insert_emoji(&self, text_view: &Self::Type);
    fn parent_move_cursor(
        &self,
        text_view: &Self::Type,
        step: MovementStep,
        count: i32,
        extend_selection: bool,
    );
    fn parent_paste_clipboard(&self, text_view: &Self::Type);
    fn parent_set_anchor(&self, text_view: &Self::Type);
    fn parent_snapshot_layer(
        &self,
        text_view: &Self::Type,
        layer: TextViewLayer,
        snapshot: Snapshot,
    );
    fn parent_toggle_overwrite(&self, text_view: &Self::Type);
}

impl<T: TextViewImpl> TextViewImplExt for T {
    fn parent_backspace(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).backspace {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_copy_clipboard(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).copy_clipboard {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_cut_clipboard(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).cut_clipboard {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_delete_from_cursor(&self, text_view: &Self::Type, type_: DeleteType, count: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).delete_from_cursor {
                f(
                    text_view.unsafe_cast_ref::<TextView>().to_glib_none().0,
                    type_.into_glib(),
                    count,
                )
            }
        }
    }

    fn parent_extend_selection(
        &self,
        text_view: &Self::Type,
        granularity: TextExtendSelection,
        location: &TextIter,
        start: &mut TextIter,
        end: &mut TextIter,
    ) -> glib::signal::Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).extend_selection {
                glib::signal::Inhibit(from_glib(f(
                    text_view.unsafe_cast_ref::<TextView>().to_glib_none().0,
                    granularity.into_glib(),
                    location.to_glib_none().0,
                    start.to_glib_none_mut().0,
                    end.to_glib_none_mut().0,
                )))
            } else {
                glib::signal::Inhibit(false)
            }
        }
    }

    fn parent_insert_at_cursor(&self, text_view: &Self::Type, text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).insert_at_cursor {
                f(
                    text_view.unsafe_cast_ref::<TextView>().to_glib_none().0,
                    text.to_glib_none().0,
                )
            }
        }
    }

    fn parent_insert_emoji(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).insert_emoji {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_move_cursor(
        &self,
        text_view: &Self::Type,
        step: MovementStep,
        count: i32,
        extend_selection: bool,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).move_cursor {
                f(
                    text_view.unsafe_cast_ref::<TextView>().to_glib_none().0,
                    step.into_glib(),
                    count,
                    extend_selection.into_glib(),
                )
            }
        }
    }

    fn parent_paste_clipboard(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).paste_clipboard {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_set_anchor(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).set_anchor {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }

    fn parent_snapshot_layer(
        &self,
        text_view: &Self::Type,
        layer: TextViewLayer,
        snapshot: Snapshot,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).snapshot_layer {
                f(
                    text_view.unsafe_cast_ref::<TextView>().to_glib_none().0,
                    layer.into_glib(),
                    snapshot.to_glib_none().0,
                )
            }
        }
    }

    fn parent_toggle_overwrite(&self, text_view: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextViewClass;
            if let Some(f) = (*parent_class).toggle_overwrite {
                f(text_view.unsafe_cast_ref::<TextView>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: TextViewImpl> IsSubclassable<T> for TextView {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.backspace = Some(text_view_backspace::<T>);
        klass.copy_clipboard = Some(text_view_copy_clipboard::<T>);
        klass.cut_clipboard = Some(text_view_cut_clipboard::<T>);
        klass.delete_from_cursor = Some(text_view_delete_from_cursor::<T>);
        klass.extend_selection = Some(text_view_extend_selection::<T>);
        klass.insert_at_cursor = Some(text_view_insert_at_cursor::<T>);
        klass.insert_emoji = Some(text_view_insert_emoji::<T>);
        klass.move_cursor = Some(text_view_move_cursor::<T>);
        klass.paste_clipboard = Some(text_view_paste_clipboard::<T>);
        klass.set_anchor = Some(text_view_set_anchor::<T>);
        klass.snapshot_layer = Some(text_view_snapshot_layer::<T>);
        klass.toggle_overwrite = Some(text_view_toggle_overwrite::<T>);
    }
}

unsafe extern "C" fn text_view_backspace<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.backspace(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_copy_clipboard<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.copy_clipboard(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_cut_clipboard<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.cut_clipboard(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_delete_from_cursor<T: TextViewImpl>(
    ptr: *mut ffi::GtkTextView,
    type_: ffi::GtkDeleteType,
    count: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.delete_from_cursor(wrap.unsafe_cast_ref(), from_glib(type_), count)
}

unsafe extern "C" fn text_view_extend_selection<T: TextViewImpl>(
    ptr: *mut ffi::GtkTextView,
    granularity: ffi::GtkTextExtendSelection,
    location: *const ffi::GtkTextIter,
    start: *mut ffi::GtkTextIter,
    end: *mut ffi::GtkTextIter,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    let mut start_copy = from_glib_none(start);
    let mut end_copy = from_glib_none(end);

    let result = imp.extend_selection(
        wrap.unsafe_cast_ref(),
        from_glib(granularity),
        &from_glib_borrow(location),
        &mut start_copy,
        &mut end_copy,
    );
    *start = *start_copy.to_glib_none().0;
    *end = *end_copy.to_glib_none().0;

    result.into_glib()
}

unsafe extern "C" fn text_view_insert_at_cursor<T: TextViewImpl>(
    ptr: *mut ffi::GtkTextView,
    text_ptr: *const libc::c_char,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);
    let text: Borrowed<glib::GString> = from_glib_borrow(text_ptr);

    imp.insert_at_cursor(wrap.unsafe_cast_ref(), text.as_str())
}

unsafe extern "C" fn text_view_insert_emoji<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.insert_emoji(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_move_cursor<T: TextViewImpl>(
    ptr: *mut ffi::GtkTextView,
    step: ffi::GtkMovementStep,
    count: i32,
    extend_selection: glib::ffi::gboolean,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.move_cursor(
        wrap.unsafe_cast_ref(),
        from_glib(step),
        count,
        from_glib(extend_selection),
    )
}

unsafe extern "C" fn text_view_paste_clipboard<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.paste_clipboard(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_set_anchor<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.set_anchor(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn text_view_snapshot_layer<T: TextViewImpl>(
    ptr: *mut ffi::GtkTextView,
    layer: ffi::GtkTextViewLayer,
    snapshot: *mut ffi::GtkSnapshot,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.snapshot_layer(
        wrap.unsafe_cast_ref(),
        from_glib(layer),
        from_glib_none(snapshot),
    )
}

unsafe extern "C" fn text_view_toggle_overwrite<T: TextViewImpl>(ptr: *mut ffi::GtkTextView) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<TextView> = from_glib_borrow(ptr);

    imp.toggle_overwrite(wrap.unsafe_cast_ref())
}
