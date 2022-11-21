// Take a look at the license at the top of the repository in the LICENSE file.

struct Callbacks;

#[gtk::template_callbacks]
impl Callbacks {
    #[template_callback]
    async fn on_clicked(_widget: gtk::Widget) -> String {
        "Hello".into()
    }
}

fn main() {}

