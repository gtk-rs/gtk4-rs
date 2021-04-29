use glib::clone;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CompositeTemplate};

#[derive(Debug, CompositeTemplate)]
#[template(file = "video_player_window.ui")]
pub struct VideoPlayerWindow {
    #[template_child(id = "video")]
    pub video: TemplateChild<gtk::Video>,
    pub dialog: gtk::FileChooserNative,
}

#[glib::object_subclass]
impl ObjectSubclass for VideoPlayerWindow {
    const NAME: &'static str = "VideoPlayerWindow";
    type Type = super::VideoPlayerWindow;
    type ParentType = gtk::ApplicationWindow;

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
        klass.install_action(
            "win.open",
            None,
            move |win, _action_name, _action_target| {
                let self_ = VideoPlayerWindow::from_instance(&win);
                self_.dialog.set_transient_for(Some(win));
                self_
                    .dialog
                    .connect_response(clone!(@weak win => move |d, response| {
                        if response == gtk::ResponseType::Accept {
                            win.set_video(d.file().unwrap());
                        }
                        d.destroy();
                    }));

                self_.dialog.show();
            },
        );
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for VideoPlayerWindow {}
impl WidgetImpl for VideoPlayerWindow {}
impl WindowImpl for VideoPlayerWindow {}
impl ApplicationWindowImpl for VideoPlayerWindow {}
