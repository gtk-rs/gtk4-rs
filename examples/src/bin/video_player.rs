//! # Video Player
//!
//! This sample demonstrates how to use GtkVideo to play videos

use glib::clone;
use glib::subclass::prelude::*;
use gtk::{gio, glib, prelude::*, CompositeTemplate};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(file = "ui/video_player.ui")]
    pub struct VideoPlayerWindow {
        #[template_child(id = "video")]
        pub video: TemplateChild<gtk::Video>,
        pub dialog: gtk::FileChooserNative,
    }

    impl ObjectSubclass for VideoPlayerWindow {
        const NAME: &'static str = "VideoPlayerWindow";
        type Type = super::VideoPlayerWindow;
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::object_subclass!();

        fn new() -> Self {
            let dialog = gtk::FileChooserNative::new(
                Some("Open File"),
                gtk::NONE_WINDOW,
                gtk::FileChooserAction::Open,
                Some("Open"),
                Some("Cancel"),
            );
            dialog.set_modal(true);

            let videos_filter = gtk::FileFilter::new();
            videos_filter.add_mime_type("video/*");
            videos_filter.set_name(Some("Video"));
            dialog.add_filter(&videos_filter);

            let audio_filter = gtk::FileFilter::new();
            audio_filter.add_mime_type("audio/*");
            audio_filter.set_name(Some("Audio"));
            dialog.add_filter(&audio_filter);

            Self {
                dialog,
                video: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for VideoPlayerWindow {
        fn constructed(&self, obj: &Self::Type) {
            obj.init_actions();
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for VideoPlayerWindow {}
    impl WindowImpl for VideoPlayerWindow {}
    impl ApplicationWindowImpl for VideoPlayerWindow {}
}

glib::wrapper! {
    pub struct VideoPlayerWindow(ObjectSubclass<imp::VideoPlayerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl VideoPlayerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create VideoPlayerWindow")
    }

    fn init_actions(&self) {
        let open = gio::SimpleAction::new("open", None);

        open.connect_activate(clone!(@weak self as win => move |_, _| {
            let self_ = imp::VideoPlayerWindow::from_instance(&win);
            self_.dialog.set_transient_for(Some(&win));
            self_.dialog.connect_response(clone!(@weak win => move |d, response| {
                if response == gtk::ResponseType::Accept {
                    win.set_video(d.get_file().unwrap());
                }
                d.destroy();
            }));

            self_.dialog.show();
        }));
        self.add_action(&open);
    }

    fn set_video(&self, video: gio::File) {
        let self_ = imp::VideoPlayerWindow::from_instance(self);
        self_.video.get().set_file(Some(&video));
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.video_player"),
        Default::default(),
    )
    .expect("Failed to initialize application");

    application.connect_activate(|app| {
        let win = VideoPlayerWindow::new(app);
        win.show();
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
