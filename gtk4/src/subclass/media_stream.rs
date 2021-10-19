// Take a look at the license at the top of the repository in the LICENSE file.

use crate::subclass::prelude::*;
use crate::MediaStream;
use glib::translate::*;
use glib::{Cast, Object};

pub trait MediaStreamImpl: MediaStreamImplExt + ObjectImpl {
    fn pause(&self, media_stream: &Self::Type) {
        self.parent_pause(media_stream)
    }

    fn play(&self, media_stream: &Self::Type) -> bool {
        self.parent_play(media_stream)
    }

    fn realize(&self, media_stream: &Self::Type, surface: gdk::Surface) {
        self.parent_realize(media_stream, surface)
    }

    fn seek(&self, media_stream: &Self::Type, timestamp: i64) {
        self.parent_seek(media_stream, timestamp)
    }

    fn unrealize(&self, media_stream: &Self::Type, surface: gdk::Surface) {
        self.parent_unrealize(media_stream, surface)
    }

    fn update_audio(&self, media_stream: &Self::Type, muted: bool, volume: f64) {
        self.parent_update_audio(media_stream, muted, volume)
    }
}

pub trait MediaStreamImplExt: ObjectSubclass {
    fn parent_pause(&self, media_stream: &Self::Type);
    fn parent_play(&self, media_stream: &Self::Type) -> bool;
    fn parent_realize(&self, media_stream: &Self::Type, surface: gdk::Surface);
    fn parent_seek(&self, media_stream: &Self::Type, timestamp: i64);
    fn parent_unrealize(&self, media_stream: &Self::Type, surface: gdk::Surface);
    fn parent_update_audio(&self, media_stream: &Self::Type, muted: bool, volume: f64);
}

impl<T: MediaStreamImpl> MediaStreamImplExt for T {
    fn parent_pause(&self, media_stream: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .pause
                .expect("No parent class impl for \"pause\"");
            f(media_stream
                .unsafe_cast_ref::<MediaStream>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_play(&self, media_stream: &Self::Type) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            if let Some(f) = (*parent_class).play {
                return from_glib(f(media_stream
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0));
            }
            false
        }
    }

    fn parent_realize(&self, media_stream: &Self::Type, surface: gdk::Surface) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .realize
                .expect("No parent class impl for \"realize\"");
            f(
                media_stream
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0,
                surface.to_glib_none().0,
            )
        }
    }

    fn parent_seek(&self, media_stream: &Self::Type, timestamp: i64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .seek
                .expect("No parent class impl for \"realize\"");
            f(
                media_stream
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0,
                timestamp,
            )
        }
    }

    fn parent_unrealize(&self, media_stream: &Self::Type, surface: gdk::Surface) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .unrealize
                .expect("No parent class impl for \"unrealize\"");
            f(
                media_stream
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0,
                surface.to_glib_none().0,
            )
        }
    }

    fn parent_update_audio(&self, media_stream: &Self::Type, muted: bool, volume: f64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GtkMediaStreamClass;
            let f = (*parent_class)
                .update_audio
                .expect("No parent class impl for \"update_audio\"");
            f(
                media_stream
                    .unsafe_cast_ref::<MediaStream>()
                    .to_glib_none()
                    .0,
                muted.into_glib(),
                volume,
            )
        }
    }
}

unsafe impl<T: MediaStreamImpl> IsSubclassable<T> for MediaStream {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);

        assert!(
            crate::rt::is_initialized(),
            "GTK has to be initialized first"
        );

        let klass = class.as_mut();
        klass.pause = Some(media_stream_pause::<T>);
        klass.play = Some(media_stream_play::<T>);
        klass.realize = Some(media_stream_realize::<T>);
        klass.seek = Some(media_stream_seek::<T>);
        klass.unrealize = Some(media_stream_unrealize::<T>);
        klass.update_audio = Some(media_stream_update_audio::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn media_stream_pause<T: MediaStreamImpl>(ptr: *mut ffi::GtkMediaStream) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.pause(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn media_stream_play<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.play(wrap.unsafe_cast_ref()).into_glib()
}

unsafe extern "C" fn media_stream_realize<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    surface: *mut gdk::ffi::GdkSurface,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.realize(wrap.unsafe_cast_ref(), from_glib_none(surface))
}

unsafe extern "C" fn media_stream_seek<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    timestamp: i64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.seek(wrap.unsafe_cast_ref(), timestamp)
}

unsafe extern "C" fn media_stream_unrealize<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    surface: *mut gdk::ffi::GdkSurface,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.unrealize(wrap.unsafe_cast_ref(), from_glib_none(surface))
}

unsafe extern "C" fn media_stream_update_audio<T: MediaStreamImpl>(
    ptr: *mut ffi::GtkMediaStream,
    muted: glib::ffi::gboolean,
    volume: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<MediaStream> = from_glib_borrow(ptr);

    imp.update_audio(wrap.unsafe_cast_ref(), from_glib(muted), volume)
}
