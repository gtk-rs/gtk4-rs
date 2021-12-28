// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::Texture;
use glib::translate::*;
use glib::IsA;

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Texture`](crate::Texture).
pub trait TextureExtManual: 'static {
    #[doc(alias = "gdk_texture_download")]
    fn download(&self, data: &mut [u8], stride: usize);
}

impl<O: IsA<Texture>> TextureExtManual for O {
    fn download(&self, data: &mut [u8], stride: usize) {
        unsafe {
            assert!(
                stride >= 4,
                "Stride for a CAIRO_FORMAT_ARGB32 should be >= 4"
            );
            assert!(
                stride as i32 >= self.as_ref().width() * 4,
                "The stride must be >= 4*width"
            );
            assert!(
                data.len() as i32 >= stride as i32 * self.as_ref().height(),
                "The data is not big enough to download the texture"
            );
            ffi::gdk_texture_download(self.as_ref().to_glib_none().0, data.as_mut_ptr(), stride);
        }
    }
}

// TODO Figure out how to implement an async variant for for_pixbuf.
impl Texture {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub async fn from_bytes_future(bytes: &glib::Bytes) -> Result<Texture, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let (sender, receiver) = futures_channel::oneshot::channel();

            let task = gio::Task::new(Texture::NONE, gio::Cancellable::NONE, move |task, _| {
                let result = task.propagate();
                let _ = sender.send(result);
            });

            let task_func = glib::clone!(@strong bytes => move |task: gio::Task<Texture>, _: Option<&Texture>, _: Option<&gio::Cancellable>| {
                let result = Texture::from_bytes(&bytes);
                task.return_result(result);
            });

            task.run_in_thread(task_func);

            receiver.await.unwrap()
        }
    }

    pub async fn from_file_future(file: &impl IsA<gio::File>) -> Result<Texture, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let (sender, receiver) = futures_channel::oneshot::channel();

            let task = gio::Task::new(
                Some(file.as_ref()),
                gio::Cancellable::NONE,
                move |task, _: Option<&gio::File>| {
                    let result = task.propagate();
                    let _ = sender.send(result);
                },
            );

            let task_func = move |task: gio::Task<Texture>,
                                  source_object: Option<&gio::File>,
                                  _: Option<&gio::Cancellable>| {
                let file = source_object.unwrap();
                let result = Texture::from_file(file);
                task.return_result(result);
            };

            task.run_in_thread(task_func);

            receiver.await.unwrap()
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    pub async fn from_filename_future(
        path: &'static (impl AsRef<std::path::Path> + std::marker::Sync),
    ) -> Result<Texture, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let (sender, receiver) = futures_channel::oneshot::channel();

            let task = gio::Task::new(Texture::NONE, gio::Cancellable::NONE, move |task, _| {
                let result = task.propagate();
                let _ = sender.send(result);
            });

            let task_func = move |task: gio::Task<Texture>,
                                  _: Option<&Texture>,
                                  _: Option<&gio::Cancellable>| {
                let result = Texture::from_filename(path);
                task.return_result(result);
            };

            task.run_in_thread(task_func);

            receiver.await.unwrap()
        }
    }

    pub async fn from_resource_future(resource_path: &'static str) -> Result<Texture, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let (sender, receiver) = futures_channel::oneshot::channel();

            let task = gio::Task::new(Texture::NONE, gio::Cancellable::NONE, move |task, _| {
                let result = task.propagate();
                let _ = sender.send(result);
            });

            let task_func = move |task: gio::Task<Texture>,
                                  _: Option<&Texture>,
                                  _: Option<&gio::Cancellable>| {
                let texture = Texture::from_resource(resource_path);
                task.return_result(Ok(texture));
            };

            task.run_in_thread(task_func);

            receiver.await.unwrap()
        }
    }
}
