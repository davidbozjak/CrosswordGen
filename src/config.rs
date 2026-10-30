use dotenvy;

pub struct Config {
    pub clue_get_api: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            clue_get_api: std::env::var("CLUE_GET_API")
                .expect("CLUE_GET_API missing"),
        }
    }
}