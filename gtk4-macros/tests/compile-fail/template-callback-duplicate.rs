// Take a look at the license at the top of the repository in the LICENSE file.
//
use gtk::subclass::prelude::*;

struct Functions {}

#[gtk::template_callbacks]
impl Functions {
    #[template_callback]
    #[template_callback]
    fn the_duplicate() {}
}

fn main() {
    gtk::init().unwrap();
    let scope = gtk::BuilderRustScope::new();
    Functions::add_callbacks_to_scope(&scope);
}
