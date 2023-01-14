// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`TextBuffer`](crate::TextBuffer).

use crate::{
    prelude::*, subclass::prelude::*, TextBuffer, TextChildAnchor, TextIter, TextMark, TextTag,
};
use glib::translate::*;

pub trait TextBufferImpl: TextBufferImplExt + ObjectImpl {
    fn apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        self.parent_apply_tag(tag, start, end)
    }
    fn begin_user_action(&self) {
        self.parent_begin_user_action()
    }
    fn changed(&self) {
        self.parent_changed()
    }
    fn delete_range(&self, start: &mut TextIter, end: &mut TextIter) {
        self.parent_delete_range(start, end)
    }
    fn end_user_action(&self) {
        self.parent_end_user_action()
    }
    fn insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor) {
        self.parent_insert_child_anchor(iter, anchor)
    }
    fn insert_paintable(&self, iter: &mut TextIter, paintable: &gdk::Paintable) {
        self.parent_insert_paintable(iter, paintable)
    }
    fn insert_text(&self, iter: &mut TextIter, new_text: &str) {
        self.parent_insert_text(iter, new_text)
    }
    fn mark_deleted(&self, mark: &TextMark) {
        self.parent_mark_deleted(mark);
    }
    fn mark_set(&self, location: &TextIter, mark: &TextMark) {
        self.parent_mark_set(location, mark)
    }
    fn modified_changed(&self) {
        self.parent_modified_changed();
    }
    fn paste_done(&self, clipboard: &gdk::Clipboard) {
        self.parent_paste_done(clipboard)
    }
    fn redo(&self) {
        self.parent_redo()
    }
    fn remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        self.parent_remove_tag(tag, start, end)
    }
    fn undo(&self) {
        self.parent_undo()
    }
}

pub trait TextBufferImplExt: ObjectSubclass {
    fn parent_apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter);
    fn parent_begin_user_action(&self);
    fn parent_changed(&self);
    fn parent_delete_range(&self, start: &mut TextIter, end: &mut TextIter);
    fn parent_end_user_action(&self);
    fn parent_insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor);
    fn parent_insert_paintable(&self, iter: &mut TextIter, paintable: &gdk::Paintable);
    fn parent_insert_text(&self, iter: &mut TextIter, new_text: &str);
    fn parent_mark_deleted(&self, mark: &TextMark);
    fn parent_mark_set(&self, location: &TextIter, mark: &TextMark);
    fn parent_modified_changed(&self);
    fn parent_paste_done(&self, clipboard: &gdk::Clipboard);
    fn parent_redo(&self);
    fn parent_remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter);
    fn parent_undo(&self);
}

impl<T: TextBufferImpl> TextBufferImplExt for T {
    fn parent_apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).apply_tag {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    tag.to_glib_none().0,
                    start.to_glib_none().0,
                    end.to_glib_none().0,
                )
            }
        }
    }

    fn parent_begin_user_action(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).begin_user_action {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }

    fn parent_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).changed {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }

    fn parent_delete_range(&self, start: &mut TextIter, end: &mut TextIter) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).delete_range {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    start.to_glib_none_mut().0,
                    end.to_glib_none_mut().0,
                )
            }
        }
    }

    fn parent_end_user_action(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).end_user_action {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }

    fn parent_insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).insert_child_anchor {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    anchor.to_glib_none().0,
                )
            }
        }
    }

    fn parent_insert_paintable(&self, iter: &mut TextIter, paintable: &gdk::Paintable) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).insert_paintable {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    paintable.to_glib_none().0,
                )
            }
        }
    }

    fn parent_insert_text(&self, iter: &mut TextIter, new_text: &str) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).insert_text {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    iter.to_glib_none_mut().0,
                    new_text.to_glib_none().0,
                    new_text.len() as i32,
                )
            }
        }
    }

    fn parent_mark_deleted(&self, mark: &TextMark) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).mark_deleted {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    mark.to_glib_none().0,
                )
            }
        }
    }

    fn parent_mark_set(&self, location: &TextIter, mark: &TextMark) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).mark_set {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    location.to_glib_none().0,
                    mark.to_glib_none().0,
                )
            }
        }
    }

    fn parent_modified_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).modified_changed {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }

    fn parent_paste_done(&self, clipboard: &gdk::Clipboard) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).paste_done {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    clipboard.to_glib_none().0,
                )
            }
        }
    }

    fn parent_redo(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).redo {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }

    fn parent_remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).remove_tag {
                f(
                    self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0,
                    tag.to_glib_none().0,
                    start.to_glib_none().0,
                    end.to_glib_none().0,
                )
            }
        }
    }

    fn parent_undo(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkTextBufferClass;
            if let Some(f) = (*parent_class).undo {
                f(self.obj().unsafe_cast_ref::<TextBuffer>().to_glib_none().0)
            }
        }
    }
}

unsafe impl<T: TextBufferImpl> IsSubclassable<T> for TextBuffer {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.apply_tag = Some(text_buffer_apply_tag::<T>);
        klass.begin_user_action = Some(text_buffer_begin_user_action::<T>);
        klass.changed = Some(text_buffer_changed::<T>);
        klass.delete_range = Some(text_buffer_delete_range::<T>);
        klass.end_user_action = Some(text_buffer_end_user_action::<T>);
        klass.insert_child_anchor = Some(text_buffer_insert_child_anchor::<T>);
        klass.insert_paintable = Some(text_buffer_insert_paintable::<T>);
        klass.insert_text = Some(text_buffer_insert_text::<T>);
        klass.mark_deleted = Some(text_buffer_mark_deleted::<T>);
        klass.mark_set = Some(text_buffer_mark_set::<T>);
        klass.modified_changed = Some(text_buffer_modified_changed::<T>);
        klass.paste_done = Some(text_buffer_paste_done::<T>);
        klass.remove_tag = Some(text_buffer_remove_tag::<T>);
        klass.redo = Some(text_buffer_redo::<T>);
        klass.undo = Some(text_buffer_undo::<T>);
    }
}

unsafe extern "C" fn text_buffer_apply_tag<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    tag_ptr: *mut ffi::GtkTextTag,
    start_ptr: *const ffi::GtkTextIter,
    end_ptr: *const ffi::GtkTextIter,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.apply_tag(
        &from_glib_borrow(tag_ptr),
        &from_glib_borrow(start_ptr),
        &from_glib_borrow(end_ptr),
    )
}

unsafe extern "C" fn text_buffer_begin_user_action<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.begin_user_action()
}

unsafe extern "C" fn text_buffer_changed<T: TextBufferImpl>(ptr: *mut ffi::GtkTextBuffer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed()
}

unsafe extern "C" fn text_buffer_delete_range<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    start_ptr: *mut ffi::GtkTextIter,
    end_ptr: *mut ffi::GtkTextIter,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let mut start_copy = from_glib_none(start_ptr);
    let mut end_copy = from_glib_none(end_ptr);

    imp.delete_range(&mut start_copy, &mut end_copy);

    *start_ptr = *start_copy.to_glib_none().0;
    *end_ptr = *end_copy.to_glib_none().0;
}

unsafe extern "C" fn text_buffer_end_user_action<T: TextBufferImpl>(ptr: *mut ffi::GtkTextBuffer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.end_user_action()
}

unsafe extern "C" fn text_buffer_insert_child_anchor<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    iter_ptr: *mut ffi::GtkTextIter,
    anchor_ptr: *mut ffi::GtkTextChildAnchor,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let mut iter = from_glib_none(iter_ptr);

    imp.insert_child_anchor(&mut iter, &from_glib_borrow(anchor_ptr));
    *iter_ptr = *iter.to_glib_none().0;
}

unsafe extern "C" fn text_buffer_insert_paintable<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    iter_ptr: *mut ffi::GtkTextIter,
    paintable_ptr: *mut gdk::ffi::GdkPaintable,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let mut iter = from_glib_none(iter_ptr);

    imp.insert_paintable(&mut iter, &from_glib_borrow(paintable_ptr));
    *iter_ptr = *iter.to_glib_none().0;
}

unsafe extern "C" fn text_buffer_insert_text<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    iter_ptr: *mut ffi::GtkTextIter,
    text_ptr: *const libc::c_char,
    _length: libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let text: Borrowed<glib::GString> = from_glib_borrow(text_ptr);

    let mut iter = from_glib_none(iter_ptr);

    imp.insert_text(&mut iter, text.as_str());
    *iter_ptr = *iter.to_glib_none().0;
}

unsafe extern "C" fn text_buffer_modified_changed<T: TextBufferImpl>(ptr: *mut ffi::GtkTextBuffer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.modified_changed()
}

unsafe extern "C" fn text_buffer_mark_deleted<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    mark: *mut ffi::GtkTextMark,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.mark_deleted(&from_glib_borrow(mark))
}

unsafe extern "C" fn text_buffer_mark_set<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    iter: *const ffi::GtkTextIter,
    mark: *mut ffi::GtkTextMark,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.mark_set(&from_glib_borrow(iter), &from_glib_borrow(mark))
}

unsafe extern "C" fn text_buffer_paste_done<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    clipboard_ptr: *mut gdk::ffi::GdkClipboard,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.paste_done(&from_glib_borrow(clipboard_ptr))
}

unsafe extern "C" fn text_buffer_redo<T: TextBufferImpl>(ptr: *mut ffi::GtkTextBuffer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.redo()
}

unsafe extern "C" fn text_buffer_remove_tag<T: TextBufferImpl>(
    ptr: *mut ffi::GtkTextBuffer,
    tag: *mut ffi::GtkTextTag,
    start: *const ffi::GtkTextIter,
    end: *const ffi::GtkTextIter,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.remove_tag(
        &from_glib_borrow(tag),
        &from_glib_borrow(start),
        &from_glib_borrow(end),
    )
}

unsafe extern "C" fn text_buffer_undo<T: TextBufferImpl>(ptr: *mut ffi::GtkTextBuffer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.undo()
}
