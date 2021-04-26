// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AxisUse, Device, Display, EventType, ModifierType, Seat, Surface, TimeCoord};
use glib::translate::*;
use glib::{StaticType, Type};
use std::fmt;
use std::mem;

glib::wrapper! {
    pub struct Event(Shared<ffi::GdkEvent>);

    match fn {
        ref => |ptr| ffi::gdk_event_ref(ptr),
        unref => |ptr| ffi::gdk_event_unref(ptr),
    }
}

pub const NONE_EVENT: Option<&Event> = None;

impl StaticType for Event {
    #[doc(alias = "gdk_event_get_type")]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_event_get_type()) }
    }
}

impl Event {
    pub fn downcast<T: EventKind>(self) -> Result<T, Event> {
        unsafe {
            if T::event_types().contains(&self.event_type()) {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: EventKind>(&self) -> Option<&T> {
        unsafe {
            if T::event_types().contains(&self.event_type()) {
                Some(&*(self as *const Event as *const T))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_events_get_angle")]
    #[doc(alias = "get_angle")]
    pub fn angle<P: AsRef<Event>>(&self, event: P) -> Option<f64> {
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
    pub fn center<P: AsRef<Event>>(&self, event: P) -> Option<(f64, f64)> {
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
    pub fn distance<P: AsRef<Event>>(&self, event: P) -> Option<f64> {
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

    #[doc(alias = "gdk_event_get_axis")]
    #[doc(alias = "get_axis")]
    pub fn axis(&self, axis_use: AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_axis(
                self.to_glib_none().0,
                axis_use.into_glib(),
                value.as_mut_ptr(),
            ));
            if ret {
                let value = value.assume_init();
                Some(value)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_event_get_device")]
    #[doc(alias = "get_device")]
    pub fn device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_event_get_device(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_display")]
    #[doc(alias = "get_display")]
    pub fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_event_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_event_type")]
    #[doc(alias = "get_event_type")]
    pub fn event_type(&self) -> EventType {
        unsafe { from_glib(ffi::gdk_event_get_event_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_history")]
    #[doc(alias = "get_history")]
    pub fn history(&self) -> Vec<TimeCoord> {
        unsafe {
            let mut out_n_coords = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                ffi::gdk_event_get_history(self.to_glib_none().0, out_n_coords.as_mut_ptr()),
                out_n_coords.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "gdk_event_get_modifier_state")]
    #[doc(alias = "get_modifier_state")]
    pub fn modifier_state(&self) -> ModifierType {
        unsafe { from_glib(ffi::gdk_event_get_modifier_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_pointer_emulated")]
    #[doc(alias = "get_pointer_emulated")]
    pub fn is_pointer_emulated(&self) -> bool {
        unsafe { from_glib(ffi::gdk_event_get_pointer_emulated(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_position")]
    #[doc(alias = "get_position")]
    pub fn position(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_position(
                self.to_glib_none().0,
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

    #[doc(alias = "gdk_event_get_seat")]
    #[doc(alias = "get_seat")]
    pub fn seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_event_get_seat(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_surface")]
    #[doc(alias = "get_surface")]
    pub fn surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_event_get_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_time")]
    #[doc(alias = "get_time")]
    pub fn time(&self) -> u32 {
        unsafe { ffi::gdk_event_get_time(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_event_triggers_context_menu")]
    pub fn triggers_context_menu(&self) -> bool {
        unsafe { from_glib(ffi::gdk_event_triggers_context_menu(self.to_glib_none().0)) }
    }
}

impl fmt::Display for Event {
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

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

pub unsafe trait EventKind:
    StaticType + FromGlibPtrFull<*mut ffi::GdkEvent> + 'static
{
    fn event_types() -> &'static [EventType];
}

macro_rules! define_event {
    ($rust_type:ident, $ffi_type:path, $ffi_type_path:path, $event_event_types:expr) => {
        // Can't use get_type here as this is not a boxed type but another fundamental type
        glib::wrapper! {
            pub struct $rust_type(Shared<$ffi_type>);

            match fn {
                ref => |ptr| ffi::gdk_event_ref(ptr as *mut ffi::GdkEvent) as *mut $ffi_type,
                unref => |ptr| ffi::gdk_event_unref(ptr as *mut ffi::GdkEvent),
            }
        }

        impl glib::StaticType for $rust_type {
            fn static_type() -> glib::Type {
                unsafe { from_glib($ffi_type_path()) }
            }
        }

        unsafe impl crate::event::EventKind for $rust_type {
            fn event_types() -> &'static [crate::EventType] {
                $event_event_types
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::Event;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Event) }
            }
        }

        impl AsRef<crate::Event> for $rust_type {
            fn as_ref(&self) -> &crate::Event {
                self.upcast_ref()
            }
        }

        #[doc(hidden)]
        impl glib::translate::FromGlibPtrFull<*mut ffi::GdkEvent> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::GdkEvent) -> Self {
                glib::translate::FromGlibPtrFull::from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> crate::Event {
                unsafe { std::mem::transmute(self) }
            }

            pub fn upcast_ref(&self) -> &crate::Event {
                &*self
            }
        }

        impl std::fmt::Debug for $rust_type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self))
            }
        }
    };
}
