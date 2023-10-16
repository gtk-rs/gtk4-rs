use gtk::{glib, subclass::prelude::*};

#[derive(Default, Debug, gtk::CompositeTemplate)]
#[template(file = "gif_paintable_window.ui")]
pub struct GifPaintableWindow {
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
}

#[glib::object_subclass]
impl ObjectSubclass for GifPaintableWindow {
    const NAME: &'static str = "GifPaintableWindow";
    type Type = super::GifPaintableWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.install_action_async(
            "win.open",
            None,
            |win, _action_name, _action_target| async move {
                if let Err(error) = win.open_file().await {
                    println!("Error loading the GIF: {error}");
                };
            },
        );
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GifPaintableWindow {}
impl WidgetImpl for GifPaintableWindow {}
impl WindowImpl for GifPaintableWindow {}
impl ApplicationWindowImpl for GifPaintableWindow {}
