// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::{PageSetup, PrintContext, PrintOperationPreview};
use glib::translate::*;
use glib::Cast;

pub trait PrintOperationPreviewImpl: ObjectImpl {
    fn ready(&self, print_operation_preview: &Self::Type, context: &PrintContext) {
        self.parent_ready(print_operation_preview, context)
    }

    fn got_page_size(
        &self,
        print_operation_preview: &Self::Type,
        context: &PrintContext,
        page_setup: &PageSetup,
    ) {
        self.parent_got_page_size(print_operation_preview, context, page_setup)
    }

    fn render_page(&self, print_operation_preview: &Self::Type, page_nr: i32);
    fn is_selected(&self, print_operation_preview: &Self::Type, page_nr: i32) -> bool;
    fn end_preview(&self, print_operation_preview: &Self::Type);
}

pub trait PrintOperationPreviewImplExt: ObjectSubclass {
    fn parent_ready(&self, print_operation_preview: &Self::Type, context: &PrintContext);
    fn parent_got_page_size(
        &self,
        print_operation_preview: &Self::Type,
        context: &PrintContext,
        page_setup: &PageSetup,
    );
    fn parent_render_page(&self, print_operation_preview: &Self::Type, page_nr: i32);
    fn parent_is_selected(&self, print_operation_preview: &Self::Type, page_nr: i32) -> bool;
    fn parent_end_preview(&self, print_operation_preview: &Self::Type);
}

impl<T: PrintOperationPreviewImpl> PrintOperationPreviewImplExt for T {
    fn parent_ready(&self, print_operation_preview: &Self::Type, context: &PrintContext) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).ready {
                func(
                    print_operation_preview
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                );
            }
        }
    }

    fn parent_got_page_size(
        &self,
        print_operation_preview: &Self::Type,
        context: &PrintContext,
        page_setup: &PageSetup,
    ) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).got_page_size {
                func(
                    print_operation_preview
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                    page_setup.to_glib_none().0,
                );
            }
        }
    }

    fn parent_render_page(&self, print_operation_preview: &Self::Type, page_nr: i32) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).render_page {
                func(
                    print_operation_preview
                        .unsafe_cast_ref::<PrintOperationPreview>()
                        .to_glib_none()
                        .0,
                    page_nr,
                );
            }
        }
    }

    fn parent_is_selected(&self, print_operation_preview: &Self::Type, page_nr: i32) -> bool {
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
                print_operation_preview
                    .unsafe_cast_ref::<PrintOperationPreview>()
                    .to_glib_none()
                    .0,
                page_nr,
            ))
        }
    }

    fn parent_end_preview(&self, print_operation_preview: &Self::Type) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data
                .as_ref()
                .parent_interface::<PrintOperationPreview>()
                as *const ffi::GtkPrintOperationPreviewIface;

            if let Some(func) = (*parent_iface).end_preview {
                func(
                    print_operation_preview
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

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

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
    let imp = instance.impl_();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.ready(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        &context,
    )
}

unsafe extern "C" fn print_operation_preview_got_page_size<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    contextptr: *mut ffi::GtkPrintContext,
    setupptr: *mut ffi::GtkPageSetup,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.impl_();

    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);

    imp.got_page_size(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        &context,
        &setup,
    )
}

unsafe extern "C" fn print_operation_preview_render_page<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.impl_();

    imp.render_page(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        page_nr,
    )
}

unsafe extern "C" fn print_operation_preview_is_selected<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
    page_nr: i32,
) -> glib::ffi::gboolean {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.impl_();

    imp.is_selected(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
        page_nr,
    )
    .into_glib()
}

unsafe extern "C" fn print_operation_preview_end_preview<T: PrintOperationPreviewImpl>(
    print_operation_preview: *mut ffi::GtkPrintOperationPreview,
) {
    let instance = &*(print_operation_preview as *mut T::Instance);
    let imp = instance.impl_();

    imp.end_preview(
        from_glib_borrow::<_, PrintOperationPreview>(print_operation_preview).unsafe_cast_ref(),
    )
}
