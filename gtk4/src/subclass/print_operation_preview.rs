// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for implementing the [`PrintOperationPreview`](crate::PrintOperationPreview) interface.

use crate::{prelude::*, subclass::prelude::*, PageSetup, PrintContext, PrintOperationPreview};
use glib::translate::*;

pub trait PrintOperationPreviewImpl: ObjectImpl {
    fn ready(&self, context: &PrintContext) {
        self.parent_ready(context)
    }

    fn got_page_size(&self, context: &PrintContext, page_setup: &PageSetup) {
        self.parent_got_page_size(context, page_setup)
    }

    fn render_page(&self, page_nr: i32);
    fn is_selected(&self, page_nr: i32) -> bool;
    fn end_preview(&self);
}

pub trait PrintOperationPreviewImplExt: ObjectSubclass {
    fn parent_ready(&self, context: &PrintContext);
    fn parent_got_page_size(&self, context: &PrintContext, page_setup: &PageSetup);
    fn parent_render_page(&self, page_nr: i32);
    fn parent_is_selected(&self, page_nr: i32) -> bool;
    fn parent_end_preview(&self);
}

impl<T: PrintOperationPreviewImpl> PrintOperationPreviewImplExt for T {
    fn parent_ready(&self, context: &PrintContext) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).ready {
                func(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                );
            }
        }
    }

    fn parent_got_page_size(&self, context: &PrintContext, page_setup: &PageSetup) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).got_page_size {
                func(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                    page_setup.to_glib_none().0,
                );
            }
        }
    }

    fn parent_render_page(&self, page_nr: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).render_page {
                func(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    page_nr,
                );
            }
        }
    }

    fn parent_is_selected(&self, page_nr: i32) -> bool {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;
            let func = (*parent_iface)
                .is_selected
                .expect("no parent \"is_selected\" implementation");

            from_glib(func(
                self.obj()
                    .unsafe_cast_ref::<PrintOperationPreview>()
                    .to_glib_none()
                    .0,
                page_nr,
            ))
        }
    }

    fn parent_end_preview(&self) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).end_preview {
                func(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                );
            }
        }
    }
}

unsafe impl<T: PrintOperationPreviewImpl> IsImplementable<T> for PrintOperationPreview {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        assert_initialized_main_thread!();

        iface.ready = Some(print_operation_preview_ready::<T>);
        iface.got_page_size = Some(print_operation_preview_got_page_size::<T>);
        iface.render_page = Some(print_operation_preview_render_page::<T>);
        iface.is_selected = Some(print_operation_preview_is_selected::<T>);
        iface.end_preview = Some(print_operation_preview_end_preview::<T>);
    }
}

unsafe extern "C" fn print_operation_preview_ready<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    contextptr: *mut ffi::GtkPrintContext,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.imp();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.ready(&context)
}

unsafe extern "C" fn print_operation_preview_got_page_size<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    contextptr: *mut ffi::GtkPrintContext,
    setupptr: *mut ffi::GtkPageSetup,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.imp();

    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);

    imp.got_page_size(&context, &setup)
}

unsafe extern "C" fn print_operation_preview_render_page<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.imp();

    imp.render_page(page_nr)
}

unsafe extern "C" fn print_operation_preview_is_selected<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) -> glib::ffi::gboolean {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.imp();

    imp.is_selected(page_nr).into_glib()
}

unsafe extern "C" fn print_operation_preview_end_preview<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.imp();

    imp.end_preview()
}
