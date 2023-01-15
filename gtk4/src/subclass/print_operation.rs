// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`PrintOperation`](crate::PrintOperation).

use crate::{
    prelude::*, subclass::prelude::*, PageSetup, PrintContext, PrintOperation,
    PrintOperationPreview, PrintOperationResult, PrintSettings, Widget, Window,
};
use glib::translate::*;

pub trait PrintOperationImpl: PrintOperationImplExt + PrintOperationPreviewImpl {
    fn begin_print(&self, context: &PrintContext) {
        self.parent_begin_print(context)
    }

    fn create_custom_widget(&self) -> Option<Widget> {
        self.parent_create_custom_widget()
    }

    fn custom_widget_apply(&self, widget: &Widget) {
        self.parent_custom_widget_apply(widget)
    }

    fn done(&self, result: PrintOperationResult) {
        self.parent_done(result)
    }

    fn draw_page(&self, context: &PrintContext, page_nr: i32) {
        self.parent_draw_page(context, page_nr)
    }

    fn end_print(&self, context: &PrintContext) {
        self.parent_end_print(context)
    }

    fn paginate(&self, context: &PrintContext) -> bool {
        self.parent_paginate(context)
    }

    fn preview(
        &self,
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool {
        self.parent_preview(preview, context, parent)
    }

    fn request_page_setup(&self, context: &PrintContext, page_nr: i32, setup: &PageSetup) {
        self.parent_request_page_setup(context, page_nr, setup)
    }

    fn status_changed(&self) {
        self.parent_status_changed()
    }

    fn update_custom_widget(&self, widget: &Widget, setup: &PageSetup, settings: &PrintSettings) {
        self.parent_update_custom_widget(widget, setup, settings)
    }
}

pub trait PrintOperationImplExt: ObjectSubclass {
    fn parent_begin_print(&self, context: &PrintContext);
    fn parent_create_custom_widget(&self) -> Option<Widget>;
    fn parent_custom_widget_apply(&self, widget: &Widget);
    fn parent_done(&self, result: PrintOperationResult);
    fn parent_draw_page(&self, context: &PrintContext, page_nr: i32);
    fn parent_end_print(&self, context: &PrintContext);
    fn parent_paginate(&self, context: &PrintContext) -> bool;
    fn parent_preview(
        &self,
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool;
    fn parent_request_page_setup(&self, context: &PrintContext, page_nr: i32, setup: &PageSetup);
    fn parent_status_changed(&self);
    fn parent_update_custom_widget(
        &self,
        widget: &Widget,
        setup: &PageSetup,
        settings: &PrintSettings,
    );
}

impl<T: PrintOperationImpl> PrintOperationImplExt for T {
    fn parent_begin_print(&self, context: &PrintContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).begin_print {
                f(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                )
            }
        }
    }

    fn parent_create_custom_widget(&self) -> Option<Widget> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).create_custom_widget {
                let ret = f(self
                    .obj()
                    .unsafe_cast_ref::<PrintOperation>()
                    .to_glib_none()
                    .0);
                Some(from_glib_full(ret))
            } else {
                None
            }
        }
    }

    fn parent_custom_widget_apply(&self, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).custom_widget_apply {
                f(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                );
            }
        }
    }

    fn parent_done(&self, result: PrintOperationResult) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).done {
                f(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    result.into_glib(),
                );
            }
        }
    }

    fn parent_draw_page(&self, context: &PrintContext, page_nr: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).draw_page {
                f(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                    page_nr,
                );
            }
        }
    }

    fn parent_end_print(&self, context: &PrintContext) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).end_print {
                f(
                    self.obj()
                        .unsafe_cast_ref::<PrintOperation>()
                        .to_glib_none()
                        .0,
                    context.to_glib_none().0,
                );
            }
        }
    }

    fn parent_paginate(&self, context: &PrintContext) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).paginate {
                from_glib(f(
                    self.obj()
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
        preview: &PrintOperationPreview,
        context: &PrintContext,
        parent: Option<&Window>,
    ) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).preview {
                from_glib(f(
                    self.obj()
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

    fn parent_request_page_setup(&self, context: &PrintContext, page_nr: i32, setup: &PageSetup) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).request_page_setup {
                f(
                    self.obj()
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

    fn parent_status_changed(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).status_changed {
                f(self
                    .obj()
                    .unsafe_cast_ref::<PrintOperation>()
                    .to_glib_none()
                    .0);
            }
        }
    }

    fn parent_update_custom_widget(
        &self,
        widget: &Widget,
        setup: &PageSetup,
        settings: &PrintSettings,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkPrintOperationClass;
            if let Some(f) = (*parent_class).update_custom_widget {
                f(
                    self.obj()
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
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

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
    let imp = instance.imp();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.begin_print(&context)
}

unsafe extern "C" fn print_operation_create_custom_widget<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
) -> *mut ffi::GtkWidget {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.create_custom_widget().into_glib_ptr()
}

unsafe extern "C" fn print_operation_custom_widget_apply<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    widgetptr: *mut ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);

    imp.custom_widget_apply(&widget)
}

unsafe extern "C" fn print_operation_done<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    resultptr: ffi::GtkPrintOperationResult,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.done(from_glib(resultptr))
}

unsafe extern "C" fn print_operation_draw_page<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
    page_nr: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.draw_page(&context, page_nr)
}

unsafe extern "C" fn print_operation_end_print<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);

    imp.end_print(&context)
}

unsafe extern "C" fn print_operation_request_page_setup<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    contextptr: *mut ffi::GtkPrintContext,
    page_nr: i32,
    setupptr: *mut ffi::GtkPageSetup,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let context: Borrowed<PrintContext> = from_glib_borrow(contextptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);

    imp.request_page_setup(&context, page_nr, &setup)
}

unsafe extern "C" fn print_operation_status_changed<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.status_changed()
}

unsafe extern "C" fn print_operation_update_custom_widget<T: PrintOperationImpl>(
    ptr: *mut ffi::GtkPrintOperation,
    widgetptr: *mut ffi::GtkWidget,
    setupptr: *mut ffi::GtkPageSetup,
    settingsptr: *mut ffi::GtkPrintSettings,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widgetptr);
    let setup: Borrowed<PageSetup> = from_glib_borrow(setupptr);
    let settings: Borrowed<PrintSettings> = from_glib_borrow(settingsptr);

    imp.update_custom_widget(&widget, &setup, &settings)
}
