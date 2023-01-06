// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, NativeDialog, ResponseType};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`NativeDialog`](crate::NativeDialog).
#[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
#[allow(deprecated)]
pub trait NativeDialogExtManual {
    // rustdoc-stripper-ignore-next
    /// Shows the dialog and returns a `Future` that resolves to the
    /// `ResponseType` on response.
    ///
    /// ```no_run
    /// # use gtk4 as gtk;
    /// use gtk::prelude::*;
    ///
    /// # async fn run() {
    /// let dialog = gtk::FileChooserNative::builder()
    ///    .title("Select a File")
    ///    .build();
    ///
    /// dialog.run_future().await;
    /// println!("Selected file: {:?}", dialog.file());
    /// dialog.destroy();
    /// # }
    /// ```
    fn run_future<'a>(&'a self) -> Pin<Box<dyn Future<Output = ResponseType> + 'a>>;

    // rustdoc-stripper-ignore-next
    /// Shows the dialog and calls the callback when a response has been received.
    ///
    /// **Important**: this function isn't blocking.
    ///
    /// ```no_run
    /// # use gtk4 as gtk;
    /// use gtk::prelude::*;
    ///
    /// let dialog = gtk::FileChooserNative::builder()
    ///    .title("Select a File")
    ///    .build();
    ///
    /// dialog.run_async(move |obj, answer| {
    ///     obj.destroy();
    ///     println!("Selected file: {:?}", obj.file());
    /// });
    /// ```
    fn run_async<F: FnOnce(&Self, ResponseType) + 'static>(&self, f: F);
}

impl<O: IsA<NativeDialog>> NativeDialogExtManual for O {
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

    fn run_async<F: FnOnce(&Self, ResponseType) + 'static>(&self, f: F) {
        let response_handler = Rc::new(RefCell::new(None));
        let response_handler_clone = response_handler.clone();
        let f = RefCell::new(Some(f));
        *response_handler.borrow_mut() = Some(self.connect_response(move |s, response_type| {
            if let Some(handler) = response_handler_clone.borrow_mut().take() {
                s.disconnect(handler);
            }
            (*f.borrow_mut()).take().expect("cannot get callback")(s, response_type);
        }));
        self.show();
    }
}
