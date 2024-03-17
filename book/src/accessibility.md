# Accessibility

Now let's go make ours applications accessibility is very simple and works by default.

### Problems can happen

Bugs in your code, your code can design fall, widgets, relations or incorrect states. You can previn it read <a class=file-link href="https://docs.gtk.org/gtk4/section-accessibility.html">Gtk documentation about accessibility</a>.

For example if you can make your Box implementation with Accessible features working you need implements **gtk::Accessible** like it example:

Filename: <a class=file-link href="https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/todo/2/task_row/mod.rs">listings/book/listings/todo/2/task_row/mod.rs</a>
```rust ,no_run,compile_fail
glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

```

## Roles

Roles like <a class=file-link href="https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/enum.AccessibleRole.html#variant.Button">AccessibleRole::Button</a> can affect presentation, keyboard focus, relationship with other elements.

When adding Switch accessible role we are saying for assistive technologies that that component will have the keyboard interactions expected by a switch.

```rust ,no_run,compile_fail

let switch = Switch::builder()
      .active(true)
      .accessible_role(AccessibleRole::Switch)
      .build();
```

For example With it code, we add description with <a class=file-link href="https://gtk-rs.org/gtk4-rs/git/docs/gtk4/accessible/enum.Property.html" >Property::Description</a> will be read when enable screen reader and move the mouse over the button.

```rust ,no_run,compile_fail
#use gtk::accessible::Property;
#use gtk::{glib, Application, Button};
#use gtk::{prelude::*, AccessibleRole, ApplicationWindow};
#
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.test.accessible")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("error")
        .margin_end(11)
        .margin_top(11)
        .margin_start(11)
        .margin_bottom(11)
        .accessible_role(AccessibleRole::Button)
        .build();
    button.update_property(&[
        Property::Description("It is accessible label to help people to understend it button"),
    ]);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("accessible example")
        .child(&button)
        .build();
    window.present();
}
```
