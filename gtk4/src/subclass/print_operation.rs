// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::PrintOperationPreviewImpl;
use crate::{
    PageSetup, PrintContext, PrintOperation, PrintOperationPreview, PrintOperationResult,
    PrintSettings, Widget, Window,
};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Object};

pub trait PrintOperationImpl: PrintOperationImplExt + PrintOperationPreviewImpl {
    fn begin_print(&self, print_operation: &Self::Type, context: &PrintContext) {
        self.parent_begin_print(print_operation, context)
    }

    fn create_custom_widget(&self, print_operation: &Self::Type) -> Option<Widget> {
        self.parent_create_custom_widget(print_operation)
    }

    fn custom_widget_apply(&self, print_operation: &Self::Type, widget: &Widget) {
        self.parent_custom_widget_apply(print_operation, widget)
    }

    fn done(&self, print_operation: &Self::Type, result: PrintOperationResult) {
        self.parent_done(print_operation, result)
    }

    fn draw_page(&self, print_operation: &Self::Type, context: &PrintContext, page_nr: i32) {
        self.parent_draw_page(print_operation, context, page_nr)
    }

    fn end_print(&self, print_operation: &Self::Type, context: &PrintContext) {
        self.parent_end_print(print_operation, context)
    }

    fn paginate(&self, print_operation: &Self::Type, context: &PrintContext) -> bool {
        self.parent_paginate(print_operation, context)
    }

    fn preview(
        &self,
        print_operation: &Self::Type,
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool {
        self.parent_preview(print_operation, preview, context, parent)
    }

    fn request_page_setup(
        &self,
        print_operation: &Self::Type,
        context: &PrintContext,
        page_nr: i32,
        setup: &PageSetup,
    ) {
        self.parent_request_page_setup(print_operation, context, page_nr, setup)
    }

    fn status_changed(&self, print_operation: &Self::Type) {
        self.parent_status_changed(print_operation)
    }

    fn update_custom_widget(
        &self,
        print_operation: &Self::Type,
        widget: &Widget,
        setup: &PageSetup,
        settings: &PrintSettings,
    ) {
        self.parent_update_custom_widget(print_operation, widget, setup, settings)
    }
}

pub trait PrintOperationImplExt: ObjectSubclass {
    fn parent_begin_print(&self, print_operation: &Self::Type, context: &PrintContext);
    fn parent_create_custom_widget(&self, print_operation: &Self::Type) -> Option<Widget>;
    fn parent_custom_widget_apply(&self, print_operation: &Self::Type, widget: &Widget);
    fn parent_done(&self, print_operation: &Self::Type, result: PrintOperationResult);
    fn parent_draw_page(&self, print_operation: &Self::Type, context: &PrintContext, page_nr: i32);
    fn parent_end_print(&self, print_operation: &Self::Type, context: &PrintContext);
    fn parent_paginate(&self, print_operation: &Self::Type, context: &PrintContext) -> bool;
    fn parent_preview(
        &self,
        print_operation: &Self::Type,
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool;
    fn parent_request_page_setup(
        &self,
        print_operation: &Self::Type,
        context: &PrintContext,
        page_nr: i32,
        setup: &PageSetup,
    );
    fn parent_status_changed(&self, print_operation: &Self::Type);
    fn parent_update_custom_widget(
        &self,
        print_operation: &Self::Type,
        widget: &Widget,
        setup: &PageSetup,
        settings: &PrintSettings,
    );
}

impl<T: PrintOperationImpl> PrintOperationImplExt for T {
    fn parent_begin_print(&self, print_operation: &Self::Type, context: &PrintContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).begin_print {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                )
            }
        }
    }

    fn parent_create_custom_widget(&self, print_operation: &Self::Type) -> Option<Widget> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).create_custom_widget {
                let ret = f(print_operation
                    .unsafe_cast_ref::<PrintOperation>()
                    .to_glib_none()
                    .0);
                Some(from_glib_full(ret))
            } else {
                None
            }
        }
    }

    fn parent_custom_widget_apply(&self, print_operation: &Self::Type, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).custom_widget_apply {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                );
            }
        }
    }

    fn parent_done(&self, print_operation: &Self::Type, result: PrintOperationResult) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).done {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    result.to_glib(),
                );
            }
        }
    }

    fn parent_draw_page(&self, print_operation: &Self::Type, context: &PrintContext, page_nr: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).draw_page {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                    page_nr,
                );
            }
        }
    }

    fn parent_end_print(&self, print_operation: &Self::Type, context: &PrintContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).end_print {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                );
            }
        }
    }

    fn parent_paginate(&self, print_operation: &Self::Type, context: &PrintContext) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).paginate {
                from_glib(f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                ))
            } else {
                // assume the number of pages is already set & pagination is not needed
                true
            }
        }
    }

    fn parent_preview(
        &self,
        print_operation: &Self::Type,
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).preview {
                from_glib(f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    preview.to_glib_none().0,
                    context.to_glib_none().0,
                    parent.to_glib_none().0,
                ))
            } else {
                // Let the default PrintOperationPreview be used
                false
            }
        }
    }

    fn parent_request_page_setup(
        &self,
        print_operation: &Self::Type,
        context: &PrintContext,
        page_nr: i32,
        setup: &PageSetup,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).request_page_setup {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                    page_nr,
                    setup.to_glib_none().0,
                );
            }
        }
    }

    fn parent_status_changed(&self, print_operation: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).status_changed {
                f(print_operation
                    .unsafe_cast_ref::<PrintOperation>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_update_custom_widget(
        &self,
        print_operation: &Self::Type,
        widget: &Widget,
        setup: &PageSetup,
        settings: &PrintSettings,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).update_custom_widget {
                f(
                    print_operation
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                    setup.to_glib_none().0,
                    settings.to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: PrintOperationImpl> IsSubclassable<T> for PrintOperation {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.begin_print = Some(print_operation_begin_print::<T>);
        klass.create_custom_widget = Some(print_operation_create_custom_widget::<T>);
        klass.custom_widget_apply = Some(print_operation_custom_widget_apply::<T>);
        klass.done = Some(print_operation_done::<T>);
        klass.draw_page = Some(print_operation_draw_page::<T>);
        klass.end_print = Some(print_operation_end_print::<T>);
        klass.request_page_setup = Some(print_operation_request_page_setup::<T>);
        klass.status_changed = Some(print_operation_status_changed::<T>);
        klass.update_custom_widget = Some(print_operation_update_custom_widget::<T>);
    }
}

unsafe extern "C" fn print_operation_begin_print<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.begin_print(wrap.unsafe_cast_ref(), &context)
}

unsafe extern "C" fn print_operation_create_custom_widget<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
) -> *mut ffi::GtkWidget {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);

    imp.create_custom_widget(wrap.unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn print_operation_custom_widget_apply<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    widgetptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.custom_widget_apply(wrap.unsafe_cast_ref(), &widget)
}

unsafe extern "C" fn print_operation_done<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    resultptr: ffi::GtkPrintOperationResult,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);

    imp.done(wrap.unsafe_cast_ref(), from_glib(resultptr))
}

unsafe extern "C" fn print_operation_draw_page<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
    page_nr: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.draw_page(wrap.unsafe_cast_ref(), &context, page_nr)
}

unsafe extern "C" fn print_operation_end_print<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.end_print(wrap.unsafe_cast_ref(), &context)
}

unsafe extern "C" fn print_operation_request_page_setup<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
    page_nr: i32,
    setupptr: *mut ffi::GtkPageSetup,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);

    imp.request_page_setup(wrap.unsafe_cast_ref(), &context, page_nr, &setup)
}

unsafe extern "C" fn print_operation_status_changed<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);

    imp.status_changed(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn print_operation_update_custom_widget<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    widgetptr: *mut ffi::GtkWidget,
    setupptr: *mut ffi::GtkPageSetup,
    settingsptr: *mut ffi::GtkPrintSettings,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<PrintOperation> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);
    let settings: Borrowed<PrintSettings> = from_glib_borrow(settingsptr);

    imp.update_custom_widget(wrap.unsafe_cast_ref(), &widget, &setup, &settings)
}
