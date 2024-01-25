use serde::Deserialize;

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

impl PokemonClient  {
    const URI: &'static str = "https://pokeapi.co/api/v2/pokemon";
    const PAGE_LIMIT: &'static str = "100";

    pub fn new() -> Self {
        Self{ page: 0 }
    }

    pub async fn get_pokemon_list(&mut self) -> Result<Vec<Pokemon>, reqwest::Error> {
        let current_offset = self.page.clone().to_string();
        let params: [(&str, &str); 2] = [
            ("limit", Self::PAGE_LIMIT),
            ("offset", current_offset.as_str()),
        ];
        let url = reqwest::Url::parse_with_params(Self::URI, params);
        let body = reqwest::get(url.unwrap())
            .await?
            .json::<ResultBody>()
            .await?;

        self.page += 10;
        Ok(body.results)
    }

}
