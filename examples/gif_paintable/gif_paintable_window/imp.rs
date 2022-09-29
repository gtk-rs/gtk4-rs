use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};

#[derive(Debug, CompositeTemplate)]
#[template(file = "gif_paintable_window.ui")]
pub struct GifPaintableWindow {
    #[template_child]
    pub picture: TemplateChild<gtk::Picture>,
    pub dialog: gtk::FileChooserNative,
}

#[glib::object_subclass]
impl ObjectSubclass for GifPaintableWindow {
    const NAME: &'static str = "GifPaintableWindow";
    type Type = super::GifPaintableWindow;
    type ParentType = gtk::ApplicationWindow;

    fn new() -> Self {
        let gif_filter = gtk::FileFilter::new();
        gif_filter.add_mime_type("image/gif");
        gif_filter.set_name(Some("GIF Image"));

        let dialog = gtk::FileChooserNative::builder()
            .title("Open File")
            .action(gtk::FileChooserAction::Open)
            .accept_label("Open")
            .cancel_label("Cancel")
            .modal(true)
            .filter(&gif_filter)
            .build();

        Self {
            dialog,
            picture: TemplateChild::default(),
        }
    }

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.install_action_async(
            "win.open",
            None,
            |win, _action_name, _action_target| async move {
                let dialog = &win.imp().dialog;
                dialog.set_transient_for(Some(&win));
                if dialog.run_future().await == gtk::ResponseType::Accept {
                    win.set_file(dialog.file().unwrap());
                }
                dialog.destroy();
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
