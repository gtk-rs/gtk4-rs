// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use CrossingMode;
use DeviceTool;
use Drop;
use Event;
use EventSequence;
use EventType;
use NotifyType;
use ScrollDirection;
use Surface;
use TimeCoord;
use TouchpadGesturePhase;
use glib::translate::*;

use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::mem;
use std::ops::Deref;
use std::ptr;

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

    pub fn get_axes(&self) -> Option<Vec<f64>> {
        unsafe {
            let mut axes = ptr::null_mut();
            let mut n_axes = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_axes(self.to_glib_none().0, &mut axes, n_axes.as_mut_ptr()));
            if ret { Some(FromGlibContainer::from_glib_none_num(axes, n_axes.assume_init() as usize)) } else { None }
        }
    }

    pub fn get_device_tool(&self) -> DeviceTool {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_device_tool(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn get_motion_history(&self) -> Vec<TimeCoord> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_event_get_motion_history(self.to_glib_none().0))
        }
    }
}

impl ButtonPressEvent {
    pub fn get_axes(&self) -> Option<Vec<f64>> {
        unsafe {
            let mut axes = ptr::null_mut();
            let mut n_axes = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_axes(self.to_glib_none().0, &mut axes, n_axes.as_mut_ptr()));
            if ret { Some(FromGlibContainer::from_glib_none_num(axes, n_axes.assume_init() as usize)) } else { None }
        }
    }

    pub fn get_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_click_count(&self) -> u32 {
        unsafe {
            let mut click_count = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_click_count(self.to_glib_none().0, click_count.as_mut_ptr()));
            let click_count = click_count.assume_init();
            if ret { Some(click_count) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_device_tool(&self) -> DeviceTool {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_device_tool(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> ButtonPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_BUTTON_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl ButtonReleaseEvent {
    pub fn get_axes(&self) -> Option<Vec<f64>> {
        unsafe {
            let mut axes = ptr::null_mut();
            let mut n_axes = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_axes(self.to_glib_none().0, &mut axes, n_axes.as_mut_ptr()));
            if ret { Some(FromGlibContainer::from_glib_none_num(axes, n_axes.assume_init() as usize)) } else { None }
        }
    }

    pub fn get_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_click_count(&self) -> u32 {
        unsafe {
            let mut click_count = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_click_count(self.to_glib_none().0, click_count.as_mut_ptr()));
            let click_count = click_count.assume_init();
            if ret { Some(click_count) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_device_tool(&self) -> DeviceTool {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_device_tool(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> ButtonReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_BUTTON_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl KeyPressEvent {
    pub fn get_key_group(&self) -> u32 {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_key_group(self.to_glib_none().0, group.as_mut_ptr()));
            let group = group.assume_init();
            if ret { Some(group) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_key_is_modifier(&self) -> bool {
        unsafe {
            let mut is_modifier = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_key_is_modifier(self.to_glib_none().0, is_modifier.as_mut_ptr()));
            let is_modifier = is_modifier.assume_init();
            if ret { Some(from_glib(is_modifier)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_keycode(&self) -> u16 {
        unsafe {
            let mut keycode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_keycode(self.to_glib_none().0, keycode.as_mut_ptr()));
            let keycode = keycode.assume_init();
            if ret { Some(keycode) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_keyval(&self) -> u32 {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_keyval(self.to_glib_none().0, keyval.as_mut_ptr()));
            let keyval = keyval.assume_init();
            if ret { Some(keyval) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_scancode(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_event_get_scancode(self.to_glib_none().0)
        }
    }

    pub fn new() -> KeyPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_KEY_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl KeyReleaseEvent {
    pub fn get_key_group(&self) -> u32 {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_key_group(self.to_glib_none().0, group.as_mut_ptr()));
            let group = group.assume_init();
            if ret { Some(group) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_key_is_modifier(&self) -> bool {
        unsafe {
            let mut is_modifier = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_key_is_modifier(self.to_glib_none().0, is_modifier.as_mut_ptr()));
            let is_modifier = is_modifier.assume_init();
            if ret { Some(from_glib(is_modifier)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_keycode(&self) -> u16 {
        unsafe {
            let mut keycode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_keycode(self.to_glib_none().0, keycode.as_mut_ptr()));
            let keycode = keycode.assume_init();
            if ret { Some(keycode) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_keyval(&self) -> u32 {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_keyval(self.to_glib_none().0, keyval.as_mut_ptr()));
            let keyval = keyval.assume_init();
            if ret { Some(keyval) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_scancode(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_event_get_scancode(self.to_glib_none().0)
        }
    }

    pub fn new() -> KeyReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_KEY_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl EnterNotifyEvent {
    pub fn get_crossing_detail(&self) -> NotifyType {
        unsafe {
            let mut detail = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_detail(self.to_glib_none().0, detail.as_mut_ptr()));
            let detail = detail.assume_init();
            if ret { Some(from_glib(detail)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_crossing_mode(&self) -> CrossingMode {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_mode(self.to_glib_none().0, mode.as_mut_ptr()));
            let mode = mode.assume_init();
            if ret { Some(from_glib(mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> EnterNotifyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_ENTER_NOTIFY))
        };
        event.try_into().unwrap()
    }
}

impl LeaveNotifyEvent {
    pub fn get_crossing_detail(&self) -> NotifyType {
        unsafe {
            let mut detail = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_detail(self.to_glib_none().0, detail.as_mut_ptr()));
            let detail = detail.assume_init();
            if ret { Some(from_glib(detail)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_crossing_mode(&self) -> CrossingMode {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_mode(self.to_glib_none().0, mode.as_mut_ptr()));
            let mode = mode.assume_init();
            if ret { Some(from_glib(mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> LeaveNotifyEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_LEAVE_NOTIFY))
        };
        event.try_into().unwrap()
    }
}

impl FocusChangeEvent {
    pub fn get_crossing_detail(&self) -> NotifyType {
        unsafe {
            let mut detail = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_detail(self.to_glib_none().0, detail.as_mut_ptr()));
            let detail = detail.assume_init();
            if ret { Some(from_glib(detail)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_crossing_mode(&self) -> CrossingMode {
        unsafe {
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_crossing_mode(self.to_glib_none().0, mode.as_mut_ptr()));
            let mode = mode.assume_init();
            if ret { Some(from_glib(mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_focus_in(&self) -> bool {
        unsafe {
            let mut focus_in = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_focus_in(self.to_glib_none().0, focus_in.as_mut_ptr()));
            let focus_in = focus_in.assume_init();
            if ret { Some(from_glib(focus_in)) } else { None }
        }.expect("Event getter failed")
    }

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
    pub fn get_drop(&self) -> Drop {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_drop(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> DragEnterEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_ENTER))
        };
        event.try_into().unwrap()
    }
}

impl DragLeaveEvent {
    pub fn get_drop(&self) -> Drop {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_drop(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> DragLeaveEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_LEAVE))
        };
        event.try_into().unwrap()
    }
}

impl DragMotionEvent {
    pub fn get_drop(&self) -> Drop {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_drop(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> DragMotionEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DRAG_MOTION))
        };
        event.try_into().unwrap()
    }
}

impl DropStartEvent {
    pub fn get_drop(&self) -> Drop {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_drop(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> DropStartEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_DROP_START))
        };
        event.try_into().unwrap()
    }
}

impl ScrollEvent {
    pub fn get_scroll_deltas(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_scroll_deltas(self.to_glib_none().0, delta_x.as_mut_ptr(), delta_y.as_mut_ptr()));
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            if ret { Some((delta_x, delta_y)) } else { None }
        }
    }

    pub fn get_scroll_direction(&self) -> Option<ScrollDirection> {
        unsafe {
            let mut direction = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_scroll_direction(self.to_glib_none().0, direction.as_mut_ptr()));
            let direction = direction.assume_init();
            if ret { Some(from_glib(direction)) } else { None }
        }
    }

    pub fn new() -> ScrollEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_SCROLL))
        };
        event.try_into().unwrap()
    }
}

impl GrabBrokenEvent {
    pub fn get_grab_surface(&self) -> Surface {
        unsafe {
            let mut surface = ptr::null_mut();
            let ret = from_glib(gdk_sys::gdk_event_get_grab_surface(self.to_glib_none().0, &mut surface));
            if ret { Some(from_glib_none(surface)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> GrabBrokenEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_GRAB_BROKEN))
        };
        event.try_into().unwrap()
    }
}

impl TouchBeginEvent {
    pub fn get_event_sequence(&self) -> EventSequence {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_event_sequence(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn get_touch_emulating_pointer(&self) -> bool {
        unsafe {
            let mut emulating = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touch_emulating_pointer(self.to_glib_none().0, emulating.as_mut_ptr()));
            let emulating = emulating.assume_init();
            if ret { Some(from_glib(emulating)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> TouchBeginEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_BEGIN))
        };
        event.try_into().unwrap()
    }
}

impl TouchUpdateEvent {
    pub fn get_event_sequence(&self) -> EventSequence {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_event_sequence(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn get_touch_emulating_pointer(&self) -> bool {
        unsafe {
            let mut emulating = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touch_emulating_pointer(self.to_glib_none().0, emulating.as_mut_ptr()));
            let emulating = emulating.assume_init();
            if ret { Some(from_glib(emulating)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> TouchUpdateEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_UPDATE))
        };
        event.try_into().unwrap()
    }
}

impl TouchEndEvent {
    pub fn get_event_sequence(&self) -> EventSequence {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_event_sequence(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn get_touch_emulating_pointer(&self) -> bool {
        unsafe {
            let mut emulating = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touch_emulating_pointer(self.to_glib_none().0, emulating.as_mut_ptr()));
            let emulating = emulating.assume_init();
            if ret { Some(from_glib(emulating)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> TouchEndEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_END))
        };
        event.try_into().unwrap()
    }
}

impl TouchCancelEvent {
    pub fn get_event_sequence(&self) -> EventSequence {
        let result: Option<_> = unsafe {
            from_glib_none(gdk_sys::gdk_event_get_event_sequence(self.to_glib_none().0))
        };
        result.expect("Event getter failed")
    }

    pub fn new() -> TouchCancelEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCH_CANCEL))
        };
        event.try_into().unwrap()
    }
}

impl TouchpadSwipeEvent {
    pub fn get_touchpad_deltas(&self) -> (f64, f64) {
        unsafe {
            let mut dx = mem::MaybeUninit::uninit();
            let mut dy = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_deltas(self.to_glib_none().0, dx.as_mut_ptr(), dy.as_mut_ptr()));
            let dx = dx.assume_init();
            let dy = dy.assume_init();
            if ret { Some((dx, dy)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_gesture_n_fingers(&self) -> u32 {
        unsafe {
            let mut n_fingers = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_gesture_n_fingers(self.to_glib_none().0, n_fingers.as_mut_ptr()));
            let n_fingers = n_fingers.assume_init();
            if ret { Some(n_fingers) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_gesture_phase(&self) -> TouchpadGesturePhase {
        unsafe {
            let mut phase = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_gesture_phase(self.to_glib_none().0, phase.as_mut_ptr()));
            let phase = phase.assume_init();
            if ret { Some(from_glib(phase)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> TouchpadSwipeEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCHPAD_SWIPE))
        };
        event.try_into().unwrap()
    }
}

impl TouchpadPinchEvent {
    pub fn get_touchpad_angle_delta(&self) -> f64 {
        unsafe {
            let mut delta = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_angle_delta(self.to_glib_none().0, delta.as_mut_ptr()));
            let delta = delta.assume_init();
            if ret { Some(delta) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_deltas(&self) -> (f64, f64) {
        unsafe {
            let mut dx = mem::MaybeUninit::uninit();
            let mut dy = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_deltas(self.to_glib_none().0, dx.as_mut_ptr(), dy.as_mut_ptr()));
            let dx = dx.assume_init();
            let dy = dy.assume_init();
            if ret { Some((dx, dy)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_gesture_n_fingers(&self) -> u32 {
        unsafe {
            let mut n_fingers = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_gesture_n_fingers(self.to_glib_none().0, n_fingers.as_mut_ptr()));
            let n_fingers = n_fingers.assume_init();
            if ret { Some(n_fingers) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_gesture_phase(&self) -> TouchpadGesturePhase {
        unsafe {
            let mut phase = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_gesture_phase(self.to_glib_none().0, phase.as_mut_ptr()));
            let phase = phase.assume_init();
            if ret { Some(from_glib(phase)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_touchpad_scale(&self) -> f64 {
        unsafe {
            let mut scale = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_touchpad_scale(self.to_glib_none().0, scale.as_mut_ptr()));
            let scale = scale.assume_init();
            if ret { Some(scale) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> TouchpadPinchEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_TOUCHPAD_PINCH))
        };
        event.try_into().unwrap()
    }
}

impl PadButtonPressEvent {
    pub fn get_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_group_mode(self.to_glib_none().0, group.as_mut_ptr(), mode.as_mut_ptr()));
            let group = group.assume_init();
            let mode = mode.assume_init();
            if ret { Some((group, mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> PadButtonPressEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_BUTTON_PRESS))
        };
        event.try_into().unwrap()
    }
}

impl PadButtonReleaseEvent {
    pub fn get_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_button(&self) -> u32 {
        unsafe {
            let mut button = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_button(self.to_glib_none().0, button.as_mut_ptr()));
            let button = button.assume_init();
            if ret { Some(button) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_group_mode(self.to_glib_none().0, group.as_mut_ptr(), mode.as_mut_ptr()));
            let group = group.assume_init();
            let mode = mode.assume_init();
            if ret { Some((group, mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> PadButtonReleaseEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_BUTTON_RELEASE))
        };
        event.try_into().unwrap()
    }
}

impl PadRingEvent {
    pub fn get_pad_axis_value(&self) -> (u32, f64) {
        unsafe {
            let mut index = mem::MaybeUninit::uninit();
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_axis_value(self.to_glib_none().0, index.as_mut_ptr(), value.as_mut_ptr()));
            let index = index.assume_init();
            let value = value.assume_init();
            if ret { Some((index, value)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_group_mode(self.to_glib_none().0, group.as_mut_ptr(), mode.as_mut_ptr()));
            let group = group.assume_init();
            let mode = mode.assume_init();
            if ret { Some((group, mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> PadRingEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_RING))
        };
        event.try_into().unwrap()
    }
}

impl PadStripEvent {
    pub fn get_pad_axis_value(&self) -> (u32, f64) {
        unsafe {
            let mut index = mem::MaybeUninit::uninit();
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_axis_value(self.to_glib_none().0, index.as_mut_ptr(), value.as_mut_ptr()));
            let index = index.assume_init();
            let value = value.assume_init();
            if ret { Some((index, value)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn get_pad_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_group_mode(self.to_glib_none().0, group.as_mut_ptr(), mode.as_mut_ptr()));
            let group = group.assume_init();
            let mode = mode.assume_init();
            if ret { Some((group, mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> PadStripEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_STRIP))
        };
        event.try_into().unwrap()
    }
}

impl PadGroupModeEvent {
    pub fn get_pad_group_mode(&self) -> (u32, u32) {
        unsafe {
            let mut group = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_event_get_pad_group_mode(self.to_glib_none().0, group.as_mut_ptr(), mode.as_mut_ptr()));
            let group = group.assume_init();
            let mode = mode.assume_init();
            if ret { Some((group, mode)) } else { None }
        }.expect("Event getter failed")
    }

    pub fn new() -> PadGroupModeEvent {
        assert_initialized_main_thread!();
        let event: Event = unsafe {
            from_glib_full(gdk_sys::gdk_event_new(gdk_sys::GDK_PAD_GROUP_MODE))
        };
        event.try_into().unwrap()
    }
}
