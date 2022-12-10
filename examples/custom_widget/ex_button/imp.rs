use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct ExButton {
    /// Reference to the child widget.
    ///
    /// In our case it's a text label for the button. Since this example only uses a
    /// `gtk::Label`, the type could've been `Option<gtk::Label>`. However, a real button might
    /// switch between a label widget and an icon widget, and in general your widget can contain
    /// arbitrary children. Thus we used `Option<gtk::Widget>` to show how to handle any widget
    /// and to make the example easier to tweak and play with.
    ///
    /// Widgets automatically store strong references to their children, added in `set_parent()`
    /// and removed in `unparent()`. Therefore, this field could be a `WeakRef<gtk::Widget>`.
    /// Using a strong reference is just a little clearer.
    child: RefCell<Option<gtk::Widget>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExButton {
    const NAME: &'static str = "ExButton";
    type Type = super::ExButton;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        // The layout manager determines how child widgets are laid out.
        klass.set_layout_manager_type::<gtk::BinLayout>();

        // Make it look like a GTK button.
        klass.set_css_name("button");

        // Make it appear as a button to accessibility tools.
        klass.set_accessible_role(gtk::AccessibleRole::Button);
    }
}

impl ObjectImpl for ExButton {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        // Create the child label.
        let label = "Hello world!";
        let child = gtk::Label::new(Some(label));
        child.set_parent(&*obj);
        *self.child.borrow_mut() = Some(child.upcast::<gtk::Widget>());

        // Make it look like a GTK button with a label (as opposed to an icon).
        obj.add_css_class("text-button");

        // Tell accessibility tools the button has a label.
        obj.update_property(&[gtk::accessible::Property::Label(label)]);

        // Connect a gesture to handle clicks.
        let gesture = gtk::GestureClick::new();
        gesture.connect_released(|gesture, _, _, _| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            println!("Button pressed!");
        });
        obj.add_controller(gesture);
    }

    fn dispose(&self) {
        // Child widgets need to be manually unparented in `dispose()`.
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}

impl WidgetImpl for ExButton {}
