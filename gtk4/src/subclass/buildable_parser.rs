use glib::translate::*;
use glib::{ffi::gpointer, GString};
use std::collections::HashMap;
use std::mem;
use std::ptr::NonNull;

#[doc(alias = "GtkBuildableParseContext")]
pub struct BuildableParseContext(NonNull<ffi::GtkBuildableParseContext>);

impl BuildableParseContext {
    pub(crate) fn from_raw(ctx: *mut ffi::GtkBuildableParseContext) -> Self {
        Self(NonNull::new(ctx).expect("Null ParseContext"))
    }

    #[doc(alias = "gtk_buildable_parse_context_get_element")]
    pub fn element(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_buildable_parse_context_get_element(
                self.0.as_ptr(),
            ))
        }
    }

    #[doc(alias = "gtk_buildable_parse_context_get_element_stack")]
    // rustdoc-stripper-ignore-next
    /// Returns a list of tags with the last item being the currently open tag.
    pub fn element_stack(&self) -> Vec<GString> {
        unsafe {
            let stack = ffi::gtk_buildable_parse_context_get_element_stack(self.0.as_ptr());
            FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(stack)
        }
    }

    pub fn position(&self) -> (i32, i32) {
        unsafe {
            let mut line_number = mem::MaybeUninit::uninit();
            let mut char_number = mem::MaybeUninit::uninit();
            ffi::gtk_buildable_parse_context_get_position(
                self.0.as_ptr(),
                line_number.as_mut_ptr(),
                char_number.as_mut_ptr(),
            );
            (line_number.assume_init(), char_number.assume_init())
        }
    }
}

#[doc(alias = "GtkBuildableParser")]
#[repr(C)]
pub struct BuildableParser(NonNull<ffi::GtkBuildableParser>);

impl BuildableParser {
    pub(crate) fn from_raw(ptr: *mut ffi::GtkBuildableParser) -> Self {
        Self(NonNull::new(ptr).expect("parser is null"))
    }

    pub(crate) fn as_ptr(&self) -> *mut ffi::GtkBuildableParser {
        self.0.as_ptr()
    }

    pub fn new<T: BuildableParserImpl>() -> Self {
        Self::from_raw(&mut ffi::GtkBuildableParser {
            start_element: Some(start_element_trampoline::<T>),
            end_element: Some(end_element_trampoline::<T>),
            text: Some(text_trampoline::<T>),
            error: Some(error_trampoline::<T>),
            padding: [
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ],
        })
    }
}

pub unsafe trait BuildableParserImpl: Sized {
    type Type;
    fn start_element(
        parser: &mut Self::Type,
        ctx: &BuildableParseContext,
        element_name: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), glib::Error>;
    fn end_element(
        parser: &mut Self::Type,
        ctx: &BuildableParseContext,
        element_name: &str,
    ) -> Result<(), glib::Error>;
    fn text(
        parser: &mut Self::Type,
        ctx: &BuildableParseContext,
        text: &str,
    ) -> Result<(), glib::Error>;
    fn error(_parser: &mut Self::Type, _ctx: &BuildableParseContext, _error: glib::Error) {}
}

unsafe extern "C" fn start_element_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    element_name: *const libc::c_char,
    attributes_names: *mut *const libc::c_char,
    attributes_values: *mut *const libc::c_char,
    user_data: gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    let ctx = BuildableParseContext::from_raw(ctx);
    let name = from_glib_borrow::<_, GString>(element_name);
    let attributes_names = FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(attributes_names);
    let attributes_values =
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(attributes_values);
    let attributes = attributes_names
        .into_iter()
        .enumerate()
        .map(|(i, key)| {
            let val: &String = attributes_values
                .get(i)
                .expect("Key {key} doesn't have a value");
            (key, val.clone())
        })
        .collect::<HashMap<String, String>>();
    let buildable_parser = user_data as *mut T::Type;
    let mut buildable_parser = &mut *(buildable_parser as *mut T::Type);
    match T::start_element(&mut buildable_parser, &ctx, &name, attributes) {
        Ok(_) => {
            *errorptr = std::ptr::null_mut();
        }
        Err(err) => {
            *errorptr = err.into_raw();
        }
    };
}

unsafe extern "C" fn end_element_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    element_name: *const libc::c_char,
    user_data: gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    let ctx = BuildableParseContext::from_raw(ctx);
    let element_name = from_glib_borrow::<_, GString>(element_name);
    let buildable_parser = user_data as *mut ffi::GtkBuildableParser as *mut T::Type;
    let mut buildable_parser = &mut *(buildable_parser as *mut T::Type);
    match T::end_element(&mut buildable_parser, &ctx, &element_name) {
        Ok(_) => {
            *errorptr = std::ptr::null_mut();
        }
        Err(err) => {
            *errorptr = err.into_raw();
        }
    }
}

unsafe extern "C" fn text_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    text: *const libc::c_char,
    _length: usize,
    user_data: gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    let ctx = BuildableParseContext::from_raw(ctx);
    let text = from_glib_borrow::<_, GString>(text);

    let buildable_parser = user_data as *mut ffi::GtkBuildableParser as *mut T::Type;
    let mut buildable_parser = &mut *(buildable_parser as *mut T::Type);
    match T::text(&mut buildable_parser, &ctx, &text) {
        Ok(_) => {
            *errorptr = std::ptr::null_mut();
        }
        Err(err) => {
            *errorptr = err.into_raw();
        }
    }
}

unsafe extern "C" fn error_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    errorptr: *mut glib::ffi::GError,
    user_data: gpointer,
) {
    let ctx = BuildableParseContext::from_raw(ctx);

    let buildable_parser = user_data as *mut ffi::GtkBuildableParser as *mut T::Type;
    let mut buildable_parser = &mut *(buildable_parser as *mut T::Type);
    T::error(&mut buildable_parser, &ctx, from_glib_full(errorptr));
}
