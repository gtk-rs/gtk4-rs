use gtk::prelude::*;
use gtk::{gdk, gio, glib};

pub mod imp {

    use super::*;
    use gdk::subclass::prelude::*;
    use std::future::Future;
    use std::pin::Pin;

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
        fn formats(&self, _provider: &Self::Type) -> gdk::ContentFormats {
            gdk::ContentFormatsBuilder::new()
                .add_mime_type("text/plain;charset=utf-8")
                .build()
        }

        fn write_mime_type_future(
            &self,
            _provider: &Self::Type,
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
                    .write_bytes_async_future(
                        &glib::Bytes::from_static(b"Hello clipboard!\0"),
                        io_priority,
                    )
                    .await
                    .map(|_| ())
            })
        }
    }
}

glib::wrapper! {
    pub struct ContentProvider(ObjectSubclass<imp::ContentProvider>) @extends gdk::ContentProvider;
}

impl ContentProvider {
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).unwrap()
    }
}

impl Default for ContentProvider {
    fn default() -> Self {
        Self::new()
    }
}

fn on_activate(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let display = window.display();

    window.connect_realize(glib::clone!(@weak display, @weak application => move |_| {
        let provider = ContentProvider::new();
        display.clipboard().set_content(Some(&provider)).unwrap();
        glib::MainContext::default().spawn_local(glib::clone!(@weak display, @weak application => async move {
            let text = display.clipboard().read_text_async_future().await.unwrap().unwrap();
            assert_eq!(text.as_str(), "Hello clipboard!");
            application.quit();
        }));
    }));

    window.present();
}

fn main() {
    let app = gtk::Application::new(Some("org.gtk.content-provider"), Default::default());
    app.connect_activate(on_activate);
    app.run();
}
