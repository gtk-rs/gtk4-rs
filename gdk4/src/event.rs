// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Event, EventType};
use glib::{translate::*, StaticType};
use std::{fmt, mem};

impl Event {
    #[inline]
    pub fn is<T: EventKind>(&self) -> bool {
        T::event_types().contains(&self.event_type())
    }

    #[inline]
    pub fn type_(&self) -> glib::Type {
        unsafe {
            let ptr = self.as_ptr();
            from_glib((*(*(ptr as *mut glib::gobject_ffi::GTypeInstance)).g_class).g_type)
        }
    }

    #[inline]
    pub fn downcast<T: EventKind>(self) -> Result<T, Event> {
        unsafe {
            if self.is::<T>() {
                Ok(from_glib_full(self.into_glib_ptr()))
            } else {
                Err(self)
            }
        }
    }

    #[inline]
    pub fn downcast_ref<T: EventKind>(&self) -> Option<&T> {
        unsafe {
            if self.is::<T>() {
                Some(&*(self as *const Event as *const T))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_events_get_angle")]
    #[doc(alias = "get_angle")]
    pub fn angle(&self, event: impl AsRef<Event>) -> Option<f64> {
        skip_assert_initialized!();
        unsafe {
            let mut angle = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_events_get_angle(
                self.to_glib_none().0,
                event.as_ref().to_glib_none().0,
                angle.as_mut_ptr(),
            ));
            if ret {
                let angle = angle.assume_init();
                Some(angle)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_events_get_center")]
    #[doc(alias = "get_center")]
    pub fn center(&self, event: impl AsRef<Event>) -> Option<(f64, f64)> {
        skip_assert_initialized!();
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_events_get_center(
                self.to_glib_none().0,
                event.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            if ret {
                let x = x.assume_init();
                let y = y.assume_init();
                Some((x, y))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_events_get_distance")]
    #[doc(alias = "get_distance")]
    pub fn distance(&self, event: impl AsRef<Event>) -> Option<f64> {
        skip_assert_initialized!();
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_events_get_distance(
                self.to_glib_none().0,
                event.as_ref().to_glib_none().0,
                distance.as_mut_ptr(),
            ));
            if ret {
                let distance = distance.assume_init();
                Some(distance)
            } else {
                None
            }
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Event")
            .field("event_type", &self.event_type())
            .field("history", &self.history())
            .field("modifier_state", &self.modifier_state())
            .field("pointer_emulated", &self.is_pointer_emulated())
            .field("position", &self.position())
            .field("time", &self.time())
            .field("triggers_context_menu", &self.triggers_context_menu())
            .finish()
    }
}

#[doc(hidden)]
impl AsRef<Event> for Event {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

// rustdoc-stripper-ignore-next
/// A common trait implemented by the various [`Event`](crate::Event) types.
///
/// # Safety
///
/// The user is not supposed to implement this trait.
pub unsafe trait EventKind:
    StaticType + FromGlibPtrFull<*mut ffi::GdkEvent> + 'static
{
    fn event_types() -> &'static [EventType];
}

macro_rules! define_event {
    ($rust_type:ident, $ffi_type:path,$event_event_types:expr) => {
        unsafe impl crate::event::EventKind for $rust_type {
            #[inline]
            fn event_types() -> &'static [crate::EventType] {
                $event_event_types
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::Event;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Event) }
            }
        }

        impl AsRef<crate::Event> for $rust_type {
            #[inline]
            fn as_ref(&self) -> &crate::Event {
                self.upcast_ref()
            }
        }

        #[doc(hidden)]
        impl glib::translate::FromGlibPtrFull<*mut ffi::GdkEvent> for $rust_type {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ffi::GdkEvent) -> Self {
                glib::translate::FromGlibPtrFull::from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            #[inline]
            pub fn upcast(self) -> crate::Event {
                unsafe { std::mem::transmute(self) }
            }

            #[inline]
            pub fn upcast_ref(&self) -> &crate::Event {
                self
            }
        }
    };
}
