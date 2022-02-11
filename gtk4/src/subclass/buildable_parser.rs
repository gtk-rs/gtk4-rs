// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;
use glib::{GString, translate::*};
use std::{collections::HashMap, mem, ptr::NonNull};

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
pub struct BuildableParser {
    parser: ffi::GtkBuildableParser,
    data: glib::ffi::gpointer,
}

impl BuildableParser {
    pub(crate) unsafe fn from_raw_parts(
        parser: ffi::GtkBuildableParser,
        data: glib::ffi::gpointer,
    ) -> Self {
        Self { parser, data }
    }

    pub(crate) fn into_raw_parts(self) -> (ffi::GtkBuildableParser, glib::ffi::gpointer) {
        (self.parser, self.data)
    }

    pub fn new<T: BuildableParserImpl>(data: T) -> Self {
        Self {
            parser: ffi::GtkBuildableParser {
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
            },
            data: Box::into_raw(Box::new(data)) as glib::ffi::gpointer,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Takes ownership of parser data previously created by [`new()`](Self::new).
    ///
    /// # Safety
    ///
    /// The `data` pointer must have been created by `BuildableParser::new::<T>()`
    /// and must not have been taken before.
    pub unsafe fn take_data<T: BuildableParserImpl>(data: glib::ffi::gpointer) -> Box<T> {
        unsafe { Box::from_raw(data as *mut T) }
    }
}

// rustdoc-stripper-ignore-next
/// # Safety
///
/// Implementations must not panic in any of the callbacks, as they are
/// called from C code that cannot handle Rust panics.
pub unsafe trait BuildableParserImpl: Sized + 'static {
    fn start_element(
        &mut self,
        ctx: &BuildableParseContext,
        element_name: &str,
        attributes: HashMap<String, String>,
    ) -> Result<(), glib::Error>;
    fn end_element(
        &mut self,
        ctx: &BuildableParseContext,
        element_name: &str,
    ) -> Result<(), glib::Error>;
    fn text(&mut self, ctx: &BuildableParseContext, text: &str) -> Result<(), glib::Error>;
    fn error(&mut self, _ctx: &BuildableParseContext, _error: glib::Error) {}
}

unsafe extern "C" fn start_element_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    element_name: *const libc::c_char,
    attributes_names: *mut *const libc::c_char,
    attributes_values: *mut *const libc::c_char,
    user_data: glib::ffi::gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    unsafe {
        let ctx = BuildableParseContext::from_raw(ctx);
        let name = from_glib_borrow::<_, GString>(element_name);
        let attributes_names =
            FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(attributes_names);
        let attributes_values =
            FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(attributes_values);
        let attributes = attributes_names
            .into_iter()
            .zip(attributes_values)
            .collect::<HashMap<String, String>>();
        let parser_data = &mut *(user_data as *mut T);
        match parser_data.start_element(&ctx, &name, attributes) {
            Ok(_) => {
                *errorptr = std::ptr::null_mut();
            }
            Err(err) => {
                *errorptr = err.into_glib_ptr();
            }
        };
    }
}

unsafe extern "C" fn end_element_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    element_name: *const libc::c_char,
    user_data: glib::ffi::gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    unsafe {
        let ctx = BuildableParseContext::from_raw(ctx);
        let element_name = from_glib_borrow::<_, GString>(element_name);
        let parser_data = &mut *(user_data as *mut T);
        match parser_data.end_element(&ctx, &element_name) {
            Ok(_) => {
                *errorptr = std::ptr::null_mut();
            }
            Err(err) => {
                *errorptr = err.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn text_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    text: *const libc::c_char,
    _length: usize,
    user_data: glib::ffi::gpointer,
    errorptr: *mut *mut glib::ffi::GError,
) {
    unsafe {
        let ctx = BuildableParseContext::from_raw(ctx);
        let text = from_glib_borrow::<_, GString>(text);

        let parser_data = &mut *(user_data as *mut T);
        match parser_data.text(&ctx, &text) {
            Ok(_) => {
                *errorptr = std::ptr::null_mut();
            }
            Err(err) => {
                *errorptr = err.into_glib_ptr();
            }
        }
    }
}

unsafe extern "C" fn error_trampoline<T: BuildableParserImpl>(
    ctx: *mut ffi::GtkBuildableParseContext,
    errorptr: *mut glib::ffi::GError,
    user_data: glib::ffi::gpointer,
) {
    unsafe {
        let ctx = BuildableParseContext::from_raw(ctx);

        let parser_data = &mut *(user_data as *mut T);
        parser_data.error(&ctx, from_glib_full(errorptr));
    }
}
