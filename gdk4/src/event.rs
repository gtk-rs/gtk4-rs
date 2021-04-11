// Take a look at the license at the top of the repository in the LICENSE file.

pub use self::{
    button::ButtonEvent, crossing::CrossingEvent, delete::DeleteEvent, dnd::DNDEvent,
    focus::FocusEvent, grab_broken::GrabBrokenEvent, key::KeyEvent, motion::MotionEvent,
    pad::PadEvent, proximity::ProximityEvent, scroll::ScrollEvent, touch::TouchEvent,
    touchpad::TouchpadEvent,
};
use crate::{
    keys::Key, AxisUse, CrossingMode, Device, Display, Drop, EventType, KeyMatch, ModifierType,
    NotifyType, ScrollDirection, Seat, Surface, TimeCoord, TouchpadGesturePhase,
};
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
            if T::event_types().contains(&self.get_event_type()) {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: EventKind>(&self) -> Option<&T> {
        unsafe {
            if T::event_types().contains(&self.get_event_type()) {
                Some(&*(self as *const Event as *const T))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gdk_events_get_angle")]
    pub fn get_angle<P: AsRef<Event>>(&self, event: P) -> Option<f64> {
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
    pub fn get_center<P: AsRef<Event>>(&self, event: P) -> Option<(f64, f64)> {
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
    pub fn get_distance<P: AsRef<Event>>(&self, event: P) -> Option<f64> {
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
    pub fn get_axis(&self, axis_use: AxisUse) -> Option<f64> {
        unsafe {
            let mut value = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_event_get_axis(
                self.to_glib_none().0,
                axis_use.to_glib(),
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
    pub fn get_device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_event_get_device(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_display")]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_event_get_display(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_event_type")]
    pub fn get_event_type(&self) -> EventType {
        unsafe { from_glib(ffi::gdk_event_get_event_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_history")]
    pub fn get_history(&self) -> Vec<TimeCoord> {
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
    pub fn get_modifier_state(&self) -> ModifierType {
        unsafe { from_glib(ffi::gdk_event_get_modifier_state(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_pointer_emulated")]
    pub fn get_pointer_emulated(&self) -> bool {
        unsafe { from_glib(ffi::gdk_event_get_pointer_emulated(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_position")]
    pub fn get_position(&self) -> Option<(f64, f64)> {
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
    pub fn get_seat(&self) -> Option<Seat> {
        unsafe { from_glib_none(ffi::gdk_event_get_seat(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_surface")]
    pub fn get_surface(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_event_get_surface(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_event_get_time")]
    pub fn get_time(&self) -> u32 {
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
            .field("event_type", &self.get_event_type())
            .field("history", &self.get_history())
            .field("modifier_state", &self.get_modifier_state())
            .field("pointer_emulated", &self.get_pointer_emulated())
            .field("position", &self.get_position())
            .field("time", &self.get_time())
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

        impl StaticType for $rust_type {
            fn static_type() -> Type {
                unsafe { from_glib($ffi_type_path()) }
            }
        }

        unsafe impl EventKind for $rust_type {
            fn event_types() -> &'static [EventType] {
                $event_event_types
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = Event;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const Event) }
            }
        }

        impl AsRef<Event> for $rust_type {
            fn as_ref(&self) -> &Event {
                self.upcast_ref()
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut ffi::GdkEvent> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::GdkEvent) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> Event {
                unsafe { mem::transmute(self) }
            }

            pub fn upcast_ref(&self) -> &Event {
                &*self
            }
        }

        impl fmt::Debug for $rust_type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}", self))
            }
        }
    };
}

mod crossing {
    use super::*;
    define_event! {
        CrossingEvent,
        ffi::GdkCrossingEvent,
        ffi::gdk_crossing_event_get_type,
        &[EventType::EnterNotify, EventType::LeaveNotify]
    }

    impl CrossingEvent {
        #[doc(alias = "gdk_crossing_event_get_detail")]
        pub fn get_detail(&self) -> NotifyType {
            unsafe { from_glib(ffi::gdk_crossing_event_get_detail(self.to_glib_none().0)) }
        }

        #[doc(alias = "gdk_crossing_event_get_focus")]
        pub fn get_focus(&self) -> bool {
            unsafe { from_glib(ffi::gdk_crossing_event_get_focus(self.to_glib_none().0)) }
        }

        #[doc(alias = "gdk_crossing_event_get_mode")]
        pub fn get_mode(&self) -> CrossingMode {
            unsafe { from_glib(ffi::gdk_crossing_event_get_mode(self.to_glib_none().0)) }
        }
    }

    impl fmt::Display for CrossingEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("PadEvent")
                .field("detail", &self.get_detail())
                .field("focus", &self.get_focus())
                .field("mode", &self.get_mode())
                .finish()
        }
    }
}

mod button {
    use super::*;

    define_event! {
        ButtonEvent,
        ffi::GdkButtonEvent,
        ffi::gdk_button_event_get_type,
        &[EventType::ButtonPress, EventType::ButtonRelease]
    }

    impl ButtonEvent {
        #[doc(alias = "gdk_button_event_get_button")]
        pub fn get_button(&self) -> u32 {
            unsafe { ffi::gdk_button_event_get_button(self.to_glib_none().0) }
        }
    }

    impl fmt::Display for ButtonEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("ButtonEvent")
                .field("button", &self.get_button())
                .finish()
        }
    }
}

mod delete {
    use super::*;

    define_event! {
        DeleteEvent,
        ffi::GdkDeleteEvent,
        ffi::gdk_delete_event_get_type,
        &[EventType::Delete]
    }

    impl fmt::Display for DeleteEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("DeleteEvent")
        }
    }
}

mod dnd {
    use super::*;

    define_event! {
        DNDEvent,
        ffi::GdkDNDEvent,
        ffi::gdk_dnd_event_get_type,
        &[EventType::DragEnter, EventType::DragLeave, EventType::DragMotion, EventType::DropStart]
    }

    impl DNDEvent {
        #[doc(alias = "gdk_dnd_event_get_drop")]
        pub fn get_drop(&self) -> Option<Drop> {
            unsafe { from_glib_none(ffi::gdk_dnd_event_get_drop(self.to_glib_none().0)) }
        }
    }

    impl fmt::Display for DNDEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("DNDEvent")
                .field("drop", &self.get_drop())
                .finish()
        }
    }
}

mod focus {
    use super::*;

    define_event! {
        FocusEvent,
        ffi::GdkFocusEvent,
        ffi::gdk_focus_event_get_type,
        &[EventType::FocusChange]
    }

    impl FocusEvent {
        #[doc(alias = "gdk_focus_event_get_in")]
        pub fn get_in(&self) -> bool {
            unsafe { from_glib(ffi::gdk_focus_event_get_in(self.to_glib_none().0)) }
        }
    }

    impl fmt::Display for FocusEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("FocusEvent")
                .field("in", &self.get_in())
                .finish()
        }
    }
}

mod grab_broken {
    use super::*;

    define_event! {
        GrabBrokenEvent,
        ffi::GdkGrabBrokenEvent,
        ffi::gdk_grab_broken_event_get_type,
        &[EventType::GrabBroken]
    }

    impl GrabBrokenEvent {
        #[doc(alias = "gdk_grab_broken_event_get_grab_surface")]
        pub fn get_grab_surface(&self) -> Option<Surface> {
            unsafe {
                from_glib_none(ffi::gdk_grab_broken_event_get_grab_surface(
                    self.to_glib_none().0,
                ))
            }
        }

        #[doc(alias = "gdk_grab_broken_event_get_implicit")]
        pub fn get_implicit(&self) -> bool {
            unsafe {
                from_glib(ffi::gdk_grab_broken_event_get_implicit(
                    self.to_glib_none().0,
                ))
            }
        }
    }

    impl fmt::Display for GrabBrokenEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("GrabBrokenEvent")
                .field("grab_surface", &self.get_grab_surface())
                .field("implicit", &self.get_implicit())
                .finish()
        }
    }
}
mod key {
    use super::*;

    define_event! {
        KeyEvent,
        ffi::GdkKeyEvent,
        ffi::gdk_key_event_get_type,
        &[EventType::KeyPress, EventType::KeyRelease]
    }

    impl KeyEvent {
        #[doc(alias = "gdk_key_event_get_consumed_modifiers")]
        pub fn get_consumed_modifiers(&self) -> ModifierType {
            unsafe {
                from_glib(ffi::gdk_key_event_get_consumed_modifiers(
                    self.to_glib_none().0,
                ))
            }
        }

        #[doc(alias = "gdk_key_event_get_keycode")]
        pub fn get_keycode(&self) -> u32 {
            unsafe { ffi::gdk_key_event_get_keycode(self.to_glib_none().0) }
        }
        #[doc(alias = "gdk_key_event_get_keyval")]
        pub fn get_keyval(&self) -> Key {
            unsafe { ffi::gdk_key_event_get_keyval(self.to_glib_none().0).into() }
        }

        #[doc(alias = "gdk_key_event_get_layout")]
        pub fn get_layout(&self) -> u32 {
            unsafe { ffi::gdk_key_event_get_layout(self.to_glib_none().0) }
        }

        #[doc(alias = "gdk_key_event_get_level")]
        pub fn get_level(&self) -> u32 {
            unsafe { ffi::gdk_key_event_get_level(self.to_glib_none().0) }
        }

        #[doc(alias = "gdk_key_event_get_match")]
        pub fn get_match(&self) -> Option<(Key, ModifierType)> {
            unsafe {
                let mut keyval = mem::MaybeUninit::uninit();
                let mut modifiers = mem::MaybeUninit::uninit();
                let ret = from_glib(ffi::gdk_key_event_get_match(
                    self.to_glib_none().0,
                    keyval.as_mut_ptr(),
                    modifiers.as_mut_ptr(),
                ));
                if ret {
                    let keyval: Key = keyval.assume_init().into();
                    let modifiers = modifiers.assume_init();
                    Some((keyval, from_glib(modifiers)))
                } else {
                    None
                }
            }
        }

        #[doc(alias = "gdk_key_event_is_modifier")]
        pub fn is_modifier(&self) -> bool {
            unsafe { from_glib(ffi::gdk_key_event_is_modifier(self.to_glib_none().0)) }
        }

        #[doc(alias = "gdk_key_event_matches")]
        pub fn matches(&self, keyval: Key, modifiers: ModifierType) -> KeyMatch {
            unsafe {
                from_glib(ffi::gdk_key_event_matches(
                    self.to_glib_none().0,
                    keyval.to_glib(),
                    modifiers.to_glib(),
                ))
            }
        }
    }

    impl fmt::Display for KeyEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("KeyEvent")
                .field("consumed_modifiers", &self.get_consumed_modifiers())
                .field("keycode", &self.get_keycode())
                .field("keyval", &self.get_keyval())
                .field("layout", &self.get_layout())
                .field("level", &self.get_level())
                .field("match", &self.get_match())
                .field("is_modifier", &self.is_modifier())
                .finish()
        }
    }
}

mod motion {
    use super::*;

    define_event! {
        MotionEvent,
        ffi::GdkMotionEvent,
        ffi::gdk_motion_event_get_type,
        &[EventType::MotionNotify]
    }

    impl fmt::Display for MotionEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("MotionEvent")
        }
    }
}

mod pad {
    use super::*;

    define_event! {
        PadEvent,
        ffi::GdkPadEvent,
        ffi::gdk_pad_event_get_type,
        &[EventType::PadButtonPress, EventType::PadButtonRelease, EventType::PadRing, EventType::PadStrip, EventType::PadGroupMode]
    }

    impl PadEvent {
        #[doc(alias = "gdk_pad_event_get_axis_value")]
        pub fn get_axis_value(&self) -> (u32, f64) {
            unsafe {
                let mut index = mem::MaybeUninit::uninit();
                let mut value = mem::MaybeUninit::uninit();
                ffi::gdk_pad_event_get_axis_value(
                    self.to_glib_none().0,
                    index.as_mut_ptr(),
                    value.as_mut_ptr(),
                );
                let index = index.assume_init();
                let value = value.assume_init();
                (index, value)
            }
        }

        #[doc(alias = "gdk_pad_event_get_button")]
        pub fn get_button(&self) -> u32 {
            unsafe { ffi::gdk_pad_event_get_button(self.to_glib_none().0) }
        }

        #[doc(alias = "gdk_pad_event_get_group_mode")]
        pub fn get_group_mode(&self) -> (u32, u32) {
            unsafe {
                let mut group = mem::MaybeUninit::uninit();
                let mut mode = mem::MaybeUninit::uninit();
                ffi::gdk_pad_event_get_group_mode(
                    self.to_glib_none().0,
                    group.as_mut_ptr(),
                    mode.as_mut_ptr(),
                );
                let group = group.assume_init();
                let mode = mode.assume_init();
                (group, mode)
            }
        }
    }

    impl fmt::Display for PadEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("PadEvent")
                .field("axis_value", &self.get_axis_value())
                .field("button", &self.get_button())
                .field("group_mode", &self.get_group_mode())
                .finish()
        }
    }
}

mod proximity {
    use super::*;

    define_event! {
        ProximityEvent,
        ffi::GdkProximityEvent,
        ffi::gdk_proximity_event_get_type,
        &[EventType::ProximityIn, EventType::ProximityOut]
    }

    impl fmt::Display for ProximityEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("ProximityEvent")
        }
    }
}

mod scroll {
    use super::*;

    define_event! {
        ScrollEvent,
        ffi::GdkScrollEvent,
        ffi::gdk_scroll_event_get_type,
        &[EventType::Scroll]
    }

    impl ScrollEvent {
        #[doc(alias = "gdk_scroll_event_get_deltas")]
        pub fn get_deltas(&self) -> (f64, f64) {
            unsafe {
                let mut delta_x = mem::MaybeUninit::uninit();
                let mut delta_y = mem::MaybeUninit::uninit();
                ffi::gdk_scroll_event_get_deltas(
                    self.to_glib_none().0,
                    delta_x.as_mut_ptr(),
                    delta_y.as_mut_ptr(),
                );
                let delta_x = delta_x.assume_init();
                let delta_y = delta_y.assume_init();
                (delta_x, delta_y)
            }
        }

        #[doc(alias = "gdk_scroll_event_get_direction")]
        pub fn get_direction(&self) -> ScrollDirection {
            unsafe { from_glib(ffi::gdk_scroll_event_get_direction(self.to_glib_none().0)) }
        }

        #[doc(alias = "gdk_scroll_event_is_stop")]
        pub fn is_stop(&self) -> bool {
            unsafe { from_glib(ffi::gdk_scroll_event_is_stop(self.to_glib_none().0)) }
        }
    }

    impl fmt::Display for ScrollEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("ScrollEvent")
                .field("deltas", &self.get_deltas())
                .field("direction", &self.get_direction())
                .field("is_stop", &self.is_stop())
                .finish()
        }
    }
}

mod touch {
    use super::*;

    define_event! {
        TouchEvent,
        ffi::GdkTouchEvent,
        ffi::gdk_touch_event_get_type,
        &[EventType::TouchBegin, EventType::TouchUpdate, EventType::TouchEnd, EventType::TouchCancel]
    }

    impl TouchEvent {
        #[doc(alias = "gdk_touch_event_get_emulating_pointer")]
        pub fn get_emulating_pointer(&self) -> bool {
            unsafe {
                from_glib(ffi::gdk_touch_event_get_emulating_pointer(
                    self.to_glib_none().0,
                ))
            }
        }
    }

    impl fmt::Display for TouchEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("TouchEvent")
                .field("emulating_pointer", &self.get_emulating_pointer())
                .finish()
        }
    }
}

mod touchpad {
    use super::*;

    define_event! {
        TouchpadEvent,
        ffi::GdkTouchpadEvent,
        ffi::gdk_touchpad_event_get_type,
        &[EventType::TouchpadSwipe, EventType::TouchpadPinch]
    }

    impl TouchpadEvent {
        #[doc(alias = "gdk_touchpad_event_get_deltas")]
        pub fn get_deltas(&self) -> (f64, f64) {
            unsafe {
                let mut dx = mem::MaybeUninit::uninit();
                let mut dy = mem::MaybeUninit::uninit();
                ffi::gdk_touchpad_event_get_deltas(
                    self.to_glib_none().0,
                    dx.as_mut_ptr(),
                    dy.as_mut_ptr(),
                );
                let dx = dx.assume_init();
                let dy = dy.assume_init();
                (dx, dy)
            }
        }

        #[doc(alias = "gdk_touchpad_event_get_gesture_phase")]
        pub fn get_gesture_phase(&self) -> TouchpadGesturePhase {
            unsafe {
                from_glib(ffi::gdk_touchpad_event_get_gesture_phase(
                    self.to_glib_none().0,
                ))
            }
        }

        #[doc(alias = "gdk_touchpad_event_get_n_fingers")]
        pub fn get_n_fingers(&self) -> u32 {
            unsafe { ffi::gdk_touchpad_event_get_n_fingers(self.to_glib_none().0) }
        }

        #[doc(alias = "gdk_touchpad_event_get_pinch_angle_delta")]
        pub fn get_pinch_angle_delta(&self) -> f64 {
            unsafe { ffi::gdk_touchpad_event_get_pinch_angle_delta(self.to_glib_none().0) }
        }

        #[doc(alias = "gdk_touchpad_event_get_pinch_scale")]
        pub fn get_pinch_scale(&self) -> f64 {
            unsafe { ffi::gdk_touchpad_event_get_pinch_scale(self.to_glib_none().0) }
        }
    }

    impl fmt::Display for TouchpadEvent {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("TouchpadEvent")
                .field("deltas", &self.get_deltas())
                .field("gesture_phase", &self.get_gesture_phase())
                .field("n_fingers", &self.get_n_fingers())
                .field("pinch_angle_delta", &self.get_pinch_angle_delta())
                .field("pinch_scale", &self.get_pinch_scale())
                .finish()
        }
    }
}
