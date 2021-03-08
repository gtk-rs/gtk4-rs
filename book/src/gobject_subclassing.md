# Subclassing

GObjects rely heavily on inheritance.
Therefore, it makes sense that if we want to create a custom GObject, this is done via subclassing.
Let's see how this works by replacing the button in our “Hello World!” app with a custom one.

First we need to create an implementation struct, that holds the state and overrides the virtual methods.
It is advised to keep it in a private module,
since it is not supposed to be used directly.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_1/src/main.rs:impl}}
```
The description of the subclassing is in `ObjectSubclass`.
You can choose any `NAME` you want, as long as it is unique within your application.
`Type` refers to the actual GObject we will create afterwards.
`ParentType` is the GObject we inherit of.
After that, we would have the option to override the virtual methods of our ancestors.
Since we only want to have a plain button for now, we override nothing.
We still have to add the empty `impl` though.

Now we describe our custom GObject.

<span class="filename">Filename: src/main.rs</span>

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

`glib::wrapper!` does the most of the work of subclassing for us.
We only have to point to the implementation struct and which ancestor GObjects we extend.
Please note that the list of ancestor GObjects, does not mention `glib::Object`.
This is because `glib::Object` is *always* the base class in the object hierarchy.

After these steps, nothing is stopping us anymore from replacing `gtk::Button` with our `CustomButton`.

<span class="filename">Filename: src/main.rs</span>

```rust,no_run
{{#rustdoc_include ../listings/gobject_subclassing_1/src/main.rs:call}}
```
We are able to use `CustomButton` as a drop-in replacement for `gtk::Button`.
This is cool, but also not very tempting to do in a real application.
For the gain of zero benefits, it did involve quite a bit of boilerplate after all.

So let's make it a bit more interesting.
`gtk::Button` does not hold much state, but we can let `CustomButton` hold a number.

<span class="filename">Filename: src/main.rs</span>

```rust ,no_run
# use gtk::glib;
# use gtk::prelude::*;
# use gtk::{Application, ApplicationWindow};
# use std::cell::RefCell;
# 
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
#     let button = CustomButton::new();
# 
#     // Add button
#     window.set_child(Some(&button));
#     window.present();
# }
```
We override `constructed` in `ObjectImpl` so that the label of the button initializes with `number`.
We also override `clicked` in `ButtonImpl` so that every click increases `number` and updates the label.

<span class="filename">Filename: src/main.rs</span>

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
#     pub struct CustomButton {
#         number: RefCell<i32>,
#     }
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
#             Self {
#                 number: RefCell::new(0),
#             }
#         }
#     }
# 
#     // Trait shared by all GObjects
#     impl ObjectImpl for CustomButton {
#         fn constructed(&self, obj: &Self::Type) {
#             self.parent_constructed(obj);
#             obj.set_label(&self.number.borrow().to_string());
#         }
#     }
# 
#     // Trait shared by all widgets
#     impl WidgetImpl for CustomButton {}
# 
#     // Trait shared by all buttons
#     impl ButtonImpl for CustomButton {
#         fn clicked(&self, button: &Self::Type) {
#             *self.number.borrow_mut() += 1;
#             button.set_label(&self.number.borrow().to_string())
#         }
#     }
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

In `on_activate` we stop calling `connect_clicked`, and that was it.
After a rebuild, the app now features our `CustomButton` with the label “0”.
Every time we click on the button, the number displayed by the label increases by 1.

<div style="text-align:center"><img src="images/gobject_subclassing.png" /></div>
