use std::{future::Future, pin::Pin};

use gtk::{gdk, gio, glib, prelude::*, subclass::prelude::*};

#[derive(Default)]
pub struct ContentProvider {}

#[glib::object_subclass]
impl ObjectSubclass for ContentProvider {
    const NAME: &'static str = "ExContentProvider";
    type Type = super::ContentProvider;
    type ParentType = gdk::ContentProvider;
}

impl ObjectImpl for ContentProvider {}

impl ContentProviderImpl for ContentProvider {
    fn formats(&self) -> gdk::ContentFormats {
        gdk::ContentFormatsBuilder::new()
            .add_mime_type("text/plain;charset=utf-8")
            .build()
    }

    fn write_mime_type_future(
        &self,
        mime_type: &str,
        stream: &gio::OutputStream,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        let stream = stream.clone();
        let mime_type = mime_type.to_string();
        Box::pin(async move {
            if mime_type != "text/plain;charset=utf-8" {
                return Err(glib::Error::new(
                    gio::IOErrorEnum::InvalidData,
                    "Unhandled mime type",
                ));
            }
            stream
                .write_bytes_future(
                    &glib::Bytes::from_static(b"Hello clipboard!\0"),
                    io_priority,
                )
                .await
                .map(|_| ())
        })
    }
}
