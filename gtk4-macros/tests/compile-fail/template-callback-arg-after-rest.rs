// Take a look at the license at the top of the repository in the LICENSE file.

use gtk::glib;
use gtk::subclass::prelude::*;

struct Functions {}

#[gtk::template_callbacks]
impl Functions {
    #[template_callback(function)]
    fn after_rest(#[rest] _values: &[glib::Value], bad: i32) {}
}

fn main() {
    gtk::init().unwrap();
    let scope = gtk::BuilderRustScope::new();
    Functions::add_callbacks_to_scope(&scope);
}
