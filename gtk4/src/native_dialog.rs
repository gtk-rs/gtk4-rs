// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::NativeDialog;
use crate::ResponseType;
use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;

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
}
