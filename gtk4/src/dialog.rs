// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Dialog, DialogExt, DialogFlags, ResponseType, Widget, WidgetExt, Window};
use glib::object::Cast;
use glib::translate::*;
use glib::{IsA, ObjectExt};
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::ptr;

impl Dialog {
    #[doc(alias = "gtk_dialog_new_with_buttons")]
    pub fn new_with_buttons<T: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&T>,
        flags: DialogFlags,
        buttons: &[(&str, ResponseType)],
    ) -> Dialog {
        assert_initialized_main_thread!();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new_with_buttons(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                ptr::null_mut(),
            ))
            .unsafe_cast()
        };

        ret.add_buttons(buttons);
        ret
    }
}

pub trait DialogExtManual: 'static {
    #[doc(alias = "gtk_dialog_add_buttons")]
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]);

    #[doc(alias = "gtk_dialog_get_response_for_widget")]
    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> ResponseType;

    // rustdoc-stripper-ignore-next
    /// Shows the dialog and returns a `Future` that resolves to the
    /// `ResponseType` on response.
    ///
    /// ```ignore
    /// let dialog = gtk::MessageDialogBuilder::new()
    ///    .buttons(gtk::ButtonsType::OkCancel)
    ///    .text("What is your answer?")
    ///    .build();
    ///
    /// let answer = dialog.run_future().await;
    /// dialog.close();
    /// println!("Answer: {:?}", answer);
    /// ```
    fn run_future<'a>(&'a self) -> Pin<Box<dyn Future<Output = ResponseType> + 'a>>;
}

impl<O: IsA<Dialog> + IsA<Widget>> DialogExtManual for O {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            //FIXME: self.add_button don't work on 1.8
            O::add_button(self, text, id);
        }
    }

    fn get_response_for_widget<P: IsA<Widget>>(&self, widget: &P) -> ResponseType {
        unsafe {
            from_glib(ffi::gtk_dialog_get_response_for_widget(
                AsRef::<Dialog>::as_ref(self).to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    fn run_future<'a>(&'a self) -> Pin<Box<dyn Future<Output = ResponseType> + 'a>> {
        Box::pin(async move {
            let (sender, receiver) = futures_channel::oneshot::channel();

            let sender = Cell::new(Some(sender));

            let response_handler = self.connect_response(move |_, response_type| {
                if let Some(m) = sender.replace(None) {
                    let _result = m.send(response_type);
                }
            });

            self.show();

            if let Ok(response) = receiver.await {
                self.disconnect(response_handler);
                response
            } else {
                ResponseType::None
            }
        })
    }
}
