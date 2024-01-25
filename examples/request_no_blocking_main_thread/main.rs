mod pokemon_client;

use glib::clone;
use gtk::glib::once_cell::sync::Lazy;
use gtk::{glib, Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow};
use gtk::{prelude::*, PositionType};
use tokio::runtime::Runtime;

use pokemon_client::{Pokemon, PokemonClient};

const APP_ID: &str = "org.gtk_rs.pokemon.list";
static RUNTIME: Lazy<Runtime> =
    Lazy::new(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."));

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
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

    let result_poke_vec = receiver.recv_blocking().unwrap();

    let list_box = ListBox::new();
    match result_poke_vec {
        Ok(poke_vec) => {
            for pokemon in poke_vec {
                let label = Label::new(Some(&pokemon.name));
                list_box.append(&label);
            }
        }
        Err(_) => {}
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_box)
        .build();

    scrolled_window.connect_edge_reached(move |_, position| {
        let mut pokemon_client = PokemonClient::new();
        if PositionType::Bottom == position {
            RUNTIME.spawn(clone!(@strong sender => async move {
                let pokemon_vec = pokemon_client.get_pokemon_list().await;
                sender.send(pokemon_vec).await.expect("The channel needs to be open.");
            }));
        }
    });

    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            if let Ok(response) = response {
                for pokemon in response {
                    let label = Label::new(Some(&pokemon.name));
                    list_box.append(&label);
                }
            } else {
                println!("bad request");
            }
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("pokemon list infinite scroll")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();
    window.present();
}
