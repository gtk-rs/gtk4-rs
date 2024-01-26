use glib::clone;
use gtk::glib::once_cell::sync::Lazy;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use serde::Deserialize;
use tokio::runtime::Runtime;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct ResultBody {
    results: Vec<Pokemon>,
}

#[derive(Debug)]
pub struct PokemonClient {
    page: u32,
}

impl PokemonClient {
    const URI: &'static str = "https://pokeapi.co/api/v2/pokemon";
    const PAGE_LIMIT: &'static str = "100";

    pub fn new() -> Self {
        Self { page: 0 }
    }

    pub async fn get_pokemon_list(&mut self) -> Result<Vec<Pokemon>, reqwest::Error> {
        let current_offset = self.page.to_string();
        let params: [(&str, &str); 2] = [
            ("limit", Self::PAGE_LIMIT),
            ("offset", current_offset.as_str()),
        ];
        let url = reqwest::Url::parse_with_params(Self::URI, params).unwrap();
        let body = reqwest::get(url).await?.json::<ResultBody>().await?;

        self.page += 10;
        Ok(body.results)
    }
}

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.gtk_rs.pokemon.list")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let (sender, receiver) = async_channel::bounded::<Result<Vec<Pokemon>, reqwest::Error>>(1);

    RUNTIME.spawn(clone!(@strong sender => async move {
        let mut pokemon_client = PokemonClient::new();
        let pokemon_vec = pokemon_client.get_pokemon_list().await;
        sender.send(pokemon_vec).await.expect("The channel needs to be open.");
    }));

    let list_box = gtk::ListBox::builder().build();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_width(360)
        .child(&list_box)
        .build();

    scrolled_window.connect_edge_reached(move |_, position| {
        let mut pokemon_client = PokemonClient::new();
        if gtk::PositionType::Bottom == position {
            RUNTIME.spawn(clone!(@strong sender => async move {
                let pokemon_vec = pokemon_client.get_pokemon_list().await;
                sender.send(pokemon_vec).await.expect("The channel needs to be open.");
            }));
        }
    });

    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            match response {
                Ok(response) => {
                    for pokemon in response {
                        let label = gtk::Label::new(Some(&pokemon.name));
                        label.set_halign(gtk::Align::Start);
                        list_box.append(&label);
                    }
                }
                Err(_) => {
                    println!("bad request");
                }
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Tokio integration")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();
    window.present();
}
