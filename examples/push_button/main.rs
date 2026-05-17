use gtk::glib;
use gtk::prelude::*;
use std::cell::Cell;
use std::io::{self, Write};
use std::rc::Rc;

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.push_button")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

// Helper function to update the terminal status line
fn update_status(status: &str) {
    // We use a fixed width to clear the previous line, then return the cursor
    // to the end of the actual text for a better visual experience.
    print!("\r{status:<30}\r{status}");
    let _ = io::stdout().flush();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Push Button Example")
        .default_width(300)
        .default_height(100)
        .build();

    let push_button = gtk::Button::builder()
        .label("Press me!")
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();

    // State to track if the button is currently "pressed" to ignore key repeat
    let is_pressed = Rc::new(Cell::new(false));

    // Handle Mouse Events
    let gesture_click = gtk::GestureClick::new();
    gesture_click.connect_pressed(glib::clone!(
        #[weak]
        push_button,
        #[strong]
        is_pressed,
        move |gesture, _n_press, _x, _y| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            is_pressed.set(true);
            push_button.set_label("Release me!");
            update_status("Button pressed (mouse)!");
        }
    ));

    gesture_click.connect_released(glib::clone!(
        #[weak]
        push_button,
        #[strong]
        is_pressed,
        move |gesture, _n_press, _x, _y| {
            gesture.set_state(gtk::EventSequenceState::Claimed);
            is_pressed.set(false);
            push_button.set_label("Press me!");
            update_status("Button released!");
        }
    ));

    push_button.add_controller(gesture_click);

    // Handle Keyboard Events
    let controller_key = gtk::EventControllerKey::new();

    let push_button_weak = push_button.downgrade();
    let is_pressed_clone = is_pressed.clone();
    controller_key.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        if keyval != gtk::gdk::Key::space {
            return glib::Propagation::Proceed;
        }

        let Some(push_button) = push_button_weak.upgrade() else {
            return glib::Propagation::Proceed;
        };

        if !is_pressed_clone.get() {
            is_pressed_clone.set(true);
            push_button.set_label("Release me!");
            // Set the ACTIVE state flag to give visual feedback (same as mouse click)
            push_button.set_state_flags(gtk::StateFlags::ACTIVE, false);
            update_status("Button pressed (keyboard)!");
        }
        glib::Propagation::Stop
    });

    let push_button_weak = push_button.downgrade();
    let is_pressed_clone = is_pressed.clone();
    controller_key.connect_key_released(move |_controller, keyval, _keycode, _state| {
        if keyval != gtk::gdk::Key::space {
            return;
        }

        let Some(push_button) = push_button_weak.upgrade() else {
            return;
        };

        is_pressed_clone.set(false);
        push_button.set_label("Press me!");
        // Remove the ACTIVE state flag when the key is released
        push_button.unset_state_flags(gtk::StateFlags::ACTIVE);
        update_status("Button released!");
    });

    push_button.add_controller(controller_key);

    window.set_child(Some(&push_button));

    window.connect_close_request(move |_| {
        print!("\r");
        glib::Propagation::Proceed
    });

    // Set initial terminal status
    update_status("Button released!");

    window.present();
}
