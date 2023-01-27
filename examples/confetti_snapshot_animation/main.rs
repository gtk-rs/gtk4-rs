mod animated_explosion;
mod confetti_widget;
mod explosion;

pub use animated_explosion::*;
pub use confetti_widget::*;
pub use explosion::*;
use graphene::Vec2;
use gtk::prelude::*;
use gtk::{glib, graphene};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.confetti_snapshot_animation"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Confetti"));
    window.set_default_size(640, 360);

    let confetti = ConfettiWidget::new();
    window.set_child(Some(&confetti));

    // To listen to click events, we need to add a `GestureClick` controller to our `ConfettiWidget`
    let ev_ctrl = gtk::GestureClick::new();
    ev_ctrl.connect_pressed(move |event, _, x, y| {
        let confetti = event.widget().downcast::<ConfettiWidget>().unwrap();
        let params = ExplosionParameters {
            quantity: 25,
            acceleration: Vec2::new(0.0, 1.0 / 1000.0),
            velocity: -500.0 / 1000.0,
            velocity_randomness: 0.65,
            shoot_angle: std::f32::consts::FRAC_PI_2,
            shoot_angle_randomness: 0.6,
            origin: Vec2::new(x as f32, y as f32),
            turbolence: 3.0 / 1000.0,
            spread: 10.0,
            damp: 0.0005,
        };
        let duration = 3000.0;
        confetti.explode(params, duration);
    });
    confetti.add_controller(ev_ctrl);
    window.show();
}
