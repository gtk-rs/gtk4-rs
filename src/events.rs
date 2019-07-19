// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Event;
use EventType;
use TimeCoord;
use glib::translate::*;

use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;

// Event subtypes

macro_rules! subtype(
    ($subtype: ident, $type: ident) => (
        #[derive(Debug, Clone)]
        pub struct $subtype(Event);

        impl Deref for $subtype {
            type Target = Event;

            fn deref(&self) -> &Event {
                &self.0
            }
        }

        impl fmt::Display for $subtype {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($subtype))
            }
        }

        impl TryFrom<Event> for $subtype {
            type Error = Event;

            fn try_from(value: Event) -> Result<Self, Event> {
                assert_initialized_main_thread!();
                if value.get_event_type() == EventType::$type {
                    Ok($subtype(value))
                }
                else {
                    Err(value)
                }
            }
        }
    );
);

subtype!(NothingEvent, Nothing);
subtype!(DeleteEvent, Delete);
subtype!(DestroyEvent, Destroy);
subtype!(MotionNotifyEvent, MotionNotify);
subtype!(ButtonPressEvent, ButtonPress);
subtype!(ButtonReleaseEvent, ButtonRelease);
subtype!(KeyPressEvent, KeyPress);
subtype!(KeyReleaseEvent, KeyRelease);
subtype!(EnterNotifyEvent, EnterNotify);
subtype!(LeaveNotifyEvent, LeaveNotify);
subtype!(FocusChangeEvent, FocusChange);
subtype!(ConfigureEvent, Configure);
subtype!(ProximityInEvent, ProximityIn);
subtype!(ProximityOutEvent, ProximityOut);
subtype!(DragEnterEvent, DragEnter);
subtype!(DragLeaveEvent, DragLeave);
subtype!(DragMotionEvent, DragMotion);
subtype!(DropStartEvent, DropStart);
subtype!(ScrollEvent, Scroll);
subtype!(GrabBrokenEvent, GrabBroken);
subtype!(TouchBeginEvent, TouchBegin);
subtype!(TouchUpdateEvent, TouchUpdate);
subtype!(TouchEndEvent, TouchEnd);
subtype!(TouchCancelEvent, TouchCancel);
subtype!(TouchpadSwipeEvent, TouchpadSwipe);
subtype!(TouchpadPinchEvent, TouchpadPinch);
subtype!(PadButtonPressEvent, PadButtonPress);
subtype!(PadButtonReleaseEvent, PadButtonRelease);
subtype!(PadRingEvent, PadRing);
subtype!(PadStripEvent, PadStrip);
subtype!(PadGroupModeEvent, PadGroupMode);

impl NothingEvent {
    pub fn new() -> NothingEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_NOTHING))
        };
        event.try_into().unwrap()
    }
}

impl DeleteEvent {
    pub fn new() -> DeleteEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DELETE))
        };
        event.try_into().unwrap()
    }
}

impl DestroyEvent {
    pub fn new() -> DestroyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DESTROY))
        };
        event.try_into().unwrap()
    }
}

impl MotionNotifyEvent {
    pub fn new() -> MotionNotifyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_MOTION_NOTIFY))
        };
        event.try_into().unwrap()
    }

    pub fn get_motion_history(&self) -> Vec<TimeCoord> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_event_get_motion_history(self.to_glib_none().0))
        }
    }
}

impl ButtonPressEvent {
    pub fn new() -> ButtonPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_BUTTON_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl ButtonReleaseEvent {
    pub fn new() -> ButtonReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_BUTTON_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl KeyPressEvent {
    pub fn new() -> KeyPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_KEY_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl KeyReleaseEvent {
    pub fn new() -> KeyReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_KEY_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl EnterNotifyEvent {
    pub fn new() -> EnterNotifyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_ENTER_NOTIFY))
        };
        event.try_into().unwrap()
    }
}

impl LeaveNotifyEvent {
    pub fn new() -> LeaveNotifyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_LEAVE_NOTIFY))
        };
        event.try_into().unwrap()
    }
}

impl FocusChangeEvent {
    pub fn new() -> FocusChangeEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_FOCUS_CHANGE))
        };
        event.try_into().unwrap()
    }
}

impl ConfigureEvent {
    pub fn new() -> ConfigureEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_CONFIGURE))
        };
        event.try_into().unwrap()
    }
}

impl ProximityInEvent {
    pub fn new() -> ProximityInEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PROXIMITY_IN))
        };
        event.try_into().unwrap()
    }
}

impl ProximityOutEvent {
    pub fn new() -> ProximityOutEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PROXIMITY_OUT))
        };
        event.try_into().unwrap()
    }
}

impl DragEnterEvent {
    pub fn new() -> DragEnterEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_ENTER))
        };
        event.try_into().unwrap()
    }
}

impl DragLeaveEvent {
    pub fn new() -> DragLeaveEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_LEAVE))
        };
        event.try_into().unwrap()
    }
}

impl DragMotionEvent {
    pub fn new() -> DragMotionEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_MOTION))
        };
        event.try_into().unwrap()
    }
}

impl DropStartEvent {
    pub fn new() -> DropStartEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DROP_START))
        };
        event.try_into().unwrap()
    }
}

impl ScrollEvent {
    pub fn new() -> ScrollEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_SCROLL))
        };
        event.try_into().unwrap()
    }
}

impl GrabBrokenEvent {
    pub fn new() -> GrabBrokenEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_GRAB_BROKEN))
        };
        event.try_into().unwrap()
    }
}

impl TouchBeginEvent {
    pub fn new() -> TouchBeginEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_BEGIN))
        };
        event.try_into().unwrap()
    }
}

impl TouchUpdateEvent {
    pub fn new() -> TouchUpdateEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_UPDATE))
        };
        event.try_into().unwrap()
    }
}

impl TouchEndEvent {
    pub fn new() -> TouchEndEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_END))
        };
        event.try_into().unwrap()
    }
}

impl TouchCancelEvent {
    pub fn new() -> TouchCancelEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_CANCEL))
        };
        event.try_into().unwrap()
    }
}

impl TouchpadSwipeEvent {
    pub fn new() -> TouchpadSwipeEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCHPAD_SWIPE))
        };
        event.try_into().unwrap()
    }
}

impl TouchpadPinchEvent {
    pub fn new() -> TouchpadPinchEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCHPAD_PINCH))
        };
        event.try_into().unwrap()
    }
}

impl PadButtonPressEvent {
    pub fn new() -> PadButtonPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_BUTTON_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl PadButtonReleaseEvent {
    pub fn new() -> PadButtonReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_BUTTON_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl PadRingEvent {
    pub fn new() -> PadRingEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_RING))
        };
        event.try_into().unwrap()
    }
}

impl PadStripEvent {
    pub fn new() -> PadStripEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_STRIP))
        };
        event.try_into().unwrap()
    }
}

impl PadGroupModeEvent {
    pub fn new() -> PadGroupModeEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_GROUP_MODE))
        };
        event.try_into().unwrap()
    }
}
