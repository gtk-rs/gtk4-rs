# Subclassing

Every GObject has its place in the inheritance tree.
If we want to create a custom GObject, it therefore makes sense that this is done via subclassing.
Let's try it by replacing the button in our “Hello World” app with a custom button.

Subclassing in `gtk-rs` is done in a two-step process.
First you create an implementation object.

```rust ,no_run
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow};
# 
// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary traits for subclassing
    use gtk::subclass::prelude::*;

    // Object holding the state
    pub struct CustomButton;

    // The central trait for subclassing a GObject
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "CustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
        type Interfaces = ();
        type Instance = glib::subclass::simple::InstanceStruct<Self>;
        type Class = glib::subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            Self {}
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {}

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {}
}
# 
# // Wrapper implementing the necessary traits for us
# glib::wrapper! {
#     pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
#         @extends gtk::Button, gtk::Widget;
# }
# 
# impl CustomButton {
#     pub fn new() -> Self {
#         glib::Object::new(&[]).expect("Failed to create Button")
#     }
#     pub fn with_label(label: &str) -> Self {
#         let button = Self::new();
#         button.set_label(label);
#         button
#     }
# }
# 
# fn main() {
#     // Create a new application
#     let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
#         .expect("Initialization failed...");
#     app.connect_activate(|app| on_activate(app));
# 
#     // Run the application
#     app.run(&std::env::args().collect::<Vec<_>>());
# }
# 
# // When the application is launched…
# fn on_activate(application: &Application) {
#     // … create a new window …
#     let window = ApplicationWindow::new(application);
# 
#     // Create a button
#     let button = CustomButton::with_label("Run stuff");
# 
#     // Connect callback
#     button.connect_clicked(move |button| {
#         // Set the label to "Hello World!" after the button has been clicked on
#         button.set_label("Hello World!");
#     });
# 
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```
Choose whatever `NAME` you want, it has to be unique though, Rust namespaces have no effect in this case.
`Type` refers to the actual GObject we will create in part two.
`ParentType` is the GObject we inherit of.
After that we would have the option to override the virtual methods of our ancestors.
Since we only want to have a plain button for now, we override nothing but still add the empty `impl`.

Now we create the GObject we will actually use in the end.

```rust ,no_run
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow};
# use std::cell::RefCell;
# 
# // Implementation of our custom GObject
# mod imp {
#     // Import parent scope
#     use super::*;
#     // Import necessary traits for subclassing
#     use gtk::subclass::prelude::*;
# 
#     // Object holding the state
#     pub struct CustomButton;
# 
#     // The central trait for subclassing a GObject
#     impl ObjectSubclass for CustomButton {
#         const NAME: &'static str = "CustomButton";
#         type Type = super::CustomButton;
#         type ParentType = gtk::Button;
#         type Interfaces = ();
#         type Instance = glib::subclass::simple::InstanceStruct<Self>;
#         type Class = glib::subclass::simple::ClassStruct<Self>;
# 
#         glib::object_subclass!();
# 
#         fn new() -> Self {
#             Self {}
#         }
#     }
# 
#     // Trait shared by all GObjects
#     impl ObjectImpl for CustomButton {}
# 
#     // Trait shared by all widgets
#     impl WidgetImpl for CustomButton {}
# 
#     // Trait shared by all buttons
#     impl ButtonImpl for CustomButton {}
# }
# 
// Wrapper implementing the necessary traits for us
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}
# 
# fn main() {
#     // Create a new application
#     let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
#         .expect("Initialization failed...");
#     app.connect_activate(|app| on_activate(app));
# 
#     // Run the application
#     app.run(&std::env::args().collect::<Vec<_>>());
# }
# 
# // When the application is launched…
# fn on_activate(application: &Application) {
#     // … create a new window …
#     let window = ApplicationWindow::new(application);
# 
#     // Create a button
#     let button = CustomButton::with_label("Run stuff");
# 
#     // Connect callback
#     button.connect_clicked(move |button| {
#         // Set the label to "Hello World!" after the button has been clicked on
#         button.set_label("Hello World!");
#     });
# 
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```

`glib::wrapper!` does the hard work of subclassing for us.
We only have to include the information, where the implementation is (in our case `imp::CustomButton`)
and which ancestor objects we extend.
Please note that we don't have to specify `glib::Object` here, since this is *always* the base class.

Nothing is stopping us anymore to replace `gtk::Button` with our `CustomButton`.

```rust ,no_run
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow};
# 
# // Implementation of our custom GObject
# mod imp {
#     // Import parent scope
#     use super::*;
#     // Import necessary traits for subclassing
#     use gtk::subclass::prelude::*;
# 
#     // Object holding the state
#     pub struct CustomButton;
# 
#     // The central trait for subclassing a GObject
#     impl ObjectSubclass for CustomButton {
#         const NAME: &'static str = "CustomButton";
#         type Type = super::CustomButton;
#         type ParentType = gtk::Button;
#         type Interfaces = ();
#         type Instance = glib::subclass::simple::InstanceStruct<Self>;
#         type Class = glib::subclass::simple::ClassStruct<Self>;
# 
#         glib::object_subclass!();
# 
#         fn new() -> Self {
#             Self {}
#         }
#     }
# 
#     // Trait shared by all GObjects
#     impl ObjectImpl for CustomButton {}
# 
#     // Trait shared by all widgets
#     impl WidgetImpl for CustomButton {}
# 
#     // Trait shared by all buttons
#     impl ButtonImpl for CustomButton {}
# }
# 
# // Wrapper implementing the necessary traits for us
# glib::wrapper! {
#     pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
#         @extends gtk::Button, gtk::Widget;
# }
# 
# impl CustomButton {
#     pub fn new() -> Self {
#         glib::Object::new(&[]).expect("Failed to create Button")
#     }
#     pub fn with_label(label: &str) -> Self {
#         let button = Self::new();
#         button.set_label(label);
#         button
#     }
# }
# 
fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create a button
    let button = CustomButton::with_label("Run stuff");

    // Connect callback
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Add button
    window.set_child(Some(&button));
    window.present();
}
```
And our "Hello World" app behaves still completely the same.



```rust ,no_run
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;

// Implementation of our custom GObject
mod imp {
    // Import parent scope
    use super::*;
    // Import necessary traits for subclassing
    use gtk::subclass::prelude::*;

    // Object holding the state
    pub struct CustomButton {
        number: RefCell<i32>,
    }

    // The central trait for subclassing a GObject
    impl ObjectSubclass for CustomButton {
        const NAME: &'static str = "CustomButton";
        type Type = super::CustomButton;
        type ParentType = gtk::Button;
        type Interfaces = ();
        type Instance = glib::subclass::simple::InstanceStruct<Self>;
        type Class = glib::subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                number: RefCell::new(0),
            }
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for CustomButton {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_label(&self.number.borrow().to_string());
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for CustomButton {}

    // Trait shared by all buttons
    impl ButtonImpl for CustomButton {
        fn clicked(&self, button: &Self::Type) {
            *self.number.borrow_mut() += 1;
            button.set_label(&self.number.borrow().to_string())
        }
    }
}

// Wrapper implementing the necessary traits for us
glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget;
}

impl CustomButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Button")
    }
    pub fn with_label(label: &str) -> Self {
        let button = Self::new();
        button.set_label(label);
        button
    }
}

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example.Devel"), Default::default())
        .expect("Initialization failed...");
    app.connect_activate(|app| on_activate(app));

    // Run the application
    app.run(&std::env::args().collect::<Vec<_>>());
}

// When the application is launched…
fn on_activate(application: &Application) {
    // … create a new window …
    let window = ApplicationWindow::new(application);

    // Create a button
    let button = CustomButton::new();

    // Add button
    window.set_child(Some(&button));
    window.present();
}
```