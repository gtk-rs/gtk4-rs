// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`MediaStream`](crate::MediaStream).

use crate::{prelude::*, subclass::prelude::*, MediaStream};
use glib::translate::*;

pub trait MediaStreamImpl: MediaStreamImplExt + ObjectImpl {
    fn pause(&self) {
        self.parent_pause()
    }

    fn play(&self) -> bool {
        self.parent_play()
    }

    fn realize(&self, surface: gdk::Surface) {
        self.parent_realize(surface)
    }

    fn seek(&self, timestamp: i64) {
        self.parent_seek(timestamp)
    }

    fn unrealize(&self, surface: gdk::Surface) {
        self.parent_unrealize(surface)
    }

    fn update_audio(&self, muted: bool, volume: f64) {
        self.parent_update_audio(muted, volume)
    }
}

pub trait MediaStreamImplExt: ObjectSubclass {
    fn parent_pause(&self);
    fn parent_play(&self) -> bool;
    fn parent_realize(&self, surface: gdk::Surface);
    fn parent_seek(&self, timestamp: i64);
    fn parent_unrealize(&self, surface: gdk::Surface);
    fn parent_update_audio(&self, muted: bool, volume: f64);
}

impl<T: MediaStreamImpl> MediaStreamImplExt for T {
    fn parent_pause(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .pause
                .expect("No parent class impl for \"pause\"");
            f(self.obj().unsafe_cast_ref::<MediaStream>().to_glib_none().0)
        }
    }

    fn parent_play(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            if let Some(f) = (*parent_class).play {
                return from_glib(f(self
                    .obj()
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0));
            }
            false
        }
    }

    fn parent_realize(&self, surface: gdk::Surface) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .realize
                .expect("No parent class impl for \"realize\"");
            f(
                self.obj().unsafe_cast_ref::<MediaStream>().to_glib_none().0,
                surface.to_glib_none().0,
            )
        }
    }

    fn parent_seek(&self, timestamp: i64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .seek
                .expect("No parent class impl for \"realize\"");
            f(
                self.obj().unsafe_cast_ref::<MediaStream>().to_glib_none().0,
                timestamp,
            )
        }
    }

    fn parent_unrealize(&self, surface: gdk::Surface) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .unrealize
                .expect("No parent class impl for \"unrealize\"");
            f(
                self.obj().unsafe_cast_ref::<MediaStream>().to_glib_none().0,
                surface.to_glib_none().0,
            )
        }
    }

    fn parent_update_audio(&self, muted: bool, volume: f64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .update_audio
                .expect("No parent class impl for \"update_audio\"");
            f(
                self.obj().unsafe_cast_ref::<MediaStream>().to_glib_none().0,
                muted.into_glib(),
                volume,
            )
        }
    }
}

unsafe impl<T: MediaStreamImpl> IsSubclassable<T> for MediaStream {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        assert_initialized_main_thread!();

        let klass = class.as_mut();
        klass.pause = Some(media_stream_pause::<T>);
        klass.play = Some(media_stream_play::<T>);
        klass.realize = Some(media_stream_realize::<T>);
        klass.seek = Some(media_stream_seek::<T>);
        klass.unrealize = Some(media_stream_unrealize::<T>);
        klass.update_audio = Some(media_stream_update_audio::<T>);
    }
}

unsafe extern "C" fn media_stream_pause<T: MediaStreamImpl>(ptr: *mut ffi::GtkMediaStream) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pause()
}

unsafe extern "C" fn media_stream_play<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.play().into_glib()
}

unsafe extern "C" fn media_stream_realize<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    surface: *mut gdk::ffi::GdkSurface,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.realize(from_glib_none(surface))
}

unsafe extern "C" fn media_stream_seek<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    timestamp: i64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.seek(timestamp)
}

unsafe extern "C" fn media_stream_unrealize<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    surface: *mut gdk::ffi::GdkSurface,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.unrealize(from_glib_none(surface))
}

unsafe extern "C" fn media_stream_update_audio<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    muted: glib::ffi::gboolean,
    volume: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.update_audio(from_glib(muted), volume)
}
