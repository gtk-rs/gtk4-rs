// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{ControlFlow, WeakRef, subclass::SignalId, translate::*};

use crate::{
    AccessibleRole, Shortcut, Widget, ffi, prelude::*, subclass::widget::WidgetActionIter,
};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Widget`](crate::Widget).
pub trait WidgetExtManual: IsA<Widget> + 'static {
    #[doc(alias = "gtk_widget_add_tick_callback")]
    fn add_tick_callback<P: Fn(&Self, &gdk::FrameClock) -> ControlFlow + 'static>(
        &self,
        callback: P,
    ) -> TickCallbackId {
        let callback_data: Box<P> = Box::new(callback);

        unsafe extern "C" fn callback_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> ControlFlow + 'static,
        >(
            widget: *mut ffi::GtkWidget,
            frame_clock: *mut gdk::ffi::GdkFrameClock,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
                let widget: Borrowed<Widget> = from_glib_borrow(widget);
                let frame_clock = from_glib_borrow(frame_clock);
                let callback: &P = &*(user_data as *mut _);
                let res = (*callback)(widget.unsafe_cast_ref(), &frame_clock);
                res.into_glib()
            }
        }
        let callback = Some(callback_func::<Self, P> as _);

        unsafe extern "C" fn notify_func<
            O: IsA<Widget>,
            P: Fn(&O, &gdk::FrameClock) -> ControlFlow + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            unsafe {
                let _callback: Box<P> = Box::from_raw(data as *mut _);
            }
        }
        let destroy_call = Some(notify_func::<Self, P> as _);

        let id = unsafe {
            ffi::gtk_widget_add_tick_callback(
                self.as_ref().to_glib_none().0,
                callback,
                Box::into_raw(callback_data) as *mut _,
                destroy_call,
            )
        };
        TickCallbackId {
            id,
            widget: self.upcast_ref().downgrade(),
        }
    }
}

impl<O: IsA<Widget>> WidgetExtManual for O {}

#[derive(Debug)]
pub struct TickCallbackId {
    id: u32,
    widget: WeakRef<Widget>,
}

impl PartialEq for TickCallbackId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl TickCallbackId {
    #[doc(alias = "gtk_widget_remove_tick_callback")]
    #[doc(alias = "remove_tick_callback")]
    pub fn remove(self) {
        if let Some(widget) = self.widget.upgrade() {
            unsafe {
                ffi::gtk_widget_remove_tick_callback(widget.to_glib_none().0, self.id);
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait containing widget class methods that can be called on any widget type
/// at runtime, without requiring a subclass.
///
/// This trait is implemented for `glib::Class<T>` for any `T: IsA<Widget>`
/// (e.g., `Class<Widget>`, `Class<TextView>`, `Class<Button>`).
///
/// # Example
///
/// ```no_run
/// # use gtk4 as gtk;
/// use gtk::prelude::*;
///
/// let class = glib::Class::<gtk::TextView>::from_type(gtk::TextView::static_type()).unwrap();
/// let trigger = gtk::ShortcutTrigger::parse_string("<Meta>c").unwrap();
/// let shortcut = gtk::Shortcut::new(Some(trigger), Some(gtk::NamedAction::new("clipboard.copy")));
/// class.add_shortcut(&shortcut);
/// ```
pub trait WidgetClassManualExt {
    #[doc(alias = "gtk_widget_class_add_shortcut")]
    fn add_shortcut(&self, shortcut: &Shortcut);

    #[doc(alias = "gtk_widget_class_add_binding_action")]
    fn add_binding_action(&self, keyval: gdk::Key, mods: gdk::ModifierType, action_name: &str);

    #[doc(alias = "gtk_widget_class_install_property_action")]
    fn install_property_action(&self, action_name: &str, property_name: &str);

    #[doc(alias = "gtk_widget_class_query_action")]
    fn query_action(&self) -> WidgetActionIter;

    #[doc(alias = "gtk_widget_class_get_activate_signal")]
    #[doc(alias = "get_activate_signal")]
    fn activate_signal(&self) -> Option<SignalId>;

    #[doc(alias = "gtk_widget_class_get_layout_manager_type")]
    #[doc(alias = "get_layout_manager_type")]
    fn layout_manager_type(&self) -> glib::Type;

    #[doc(alias = "gtk_widget_class_get_css_name")]
    #[doc(alias = "get_css_name")]
    fn css_name(&self) -> glib::GString;

    #[doc(alias = "gtk_widget_class_get_accessible_role")]
    #[doc(alias = "get_accessible_role")]
    fn accessible_role(&self) -> AccessibleRole;
}

impl<T: IsA<Widget> + glib::object::IsClass> WidgetClassManualExt for glib::Class<T> {
    fn add_shortcut(&self, shortcut: &Shortcut) {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_add_shortcut(widget_class, shortcut.to_glib_none().0);
        }
    }

    fn add_binding_action(&self, keyval: gdk::Key, mods: gdk::ModifierType, action_name: &str) {
        let shortcut = Shortcut::new(
            Some(crate::KeyvalTrigger::new(keyval, mods)),
            Some(crate::NamedAction::new(action_name)),
        );
        self.add_shortcut(&shortcut);
    }

    fn install_property_action(&self, action_name: &str, property_name: &str) {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            ffi::gtk_widget_class_install_property_action(
                widget_class,
                action_name.to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    fn query_action(&self) -> WidgetActionIter {
        let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
        WidgetActionIter::new(widget_class)
    }

    fn activate_signal(&self) -> Option<SignalId> {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            let signal_id = ffi::gtk_widget_class_get_activate_signal(widget_class);
            if signal_id == 0 {
                None
            } else {
                Some(from_glib(signal_id))
            }
        }
    }

    fn layout_manager_type(&self) -> glib::Type {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            from_glib(ffi::gtk_widget_class_get_layout_manager_type(widget_class))
        }
    }

    fn css_name(&self) -> glib::GString {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            from_glib_none(ffi::gtk_widget_class_get_css_name(widget_class))
        }
    }

    fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            let widget_class = self as *const Self as *mut ffi::GtkWidgetClass;
            from_glib(ffi::gtk_widget_class_get_accessible_role(widget_class))
        }
    }
}
