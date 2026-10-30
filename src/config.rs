use dotenvy;

pub struct Config {
    pub clue_get_api: String,
    pub ai_gen_url: String,
    pub ai_gen_key: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        Self {
            clue_get_api: std::env::var("CLUE_GET_API")
                .expect("CLUE_GET_API missing"),
            ai_gen_url: std::env::var("AI_GEN_URL")
                .expect("CLUE_GET_API missing"),
            ai_gen_key: std::env::var("AI_GEN_KEY")
                .expect("CLUE_GET_API missing"),
        }
    }
}