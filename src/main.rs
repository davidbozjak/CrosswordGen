mod crossword;
mod entry;
mod random_provider;
mod crossword_printer;
mod html_generator;
mod crossword_eval;
mod helpers;
mod clue_fetcher;
mod config;

use crossword::Crossword;
use config::Config;
use entry::Entry;
use random_provider::DefaultRandomProvider;
use std::fs;
use reqwest::blocking::get;

fn main() {
    let config = Config::load();

    let entries_apiresult = get_data(&config);

    if entries_apiresult.is_err()
    {
        print!("API error, end");
        return;
    }

    let entries = entries_apiresult.unwrap();

    // generate 5 solutions and keep only the best one
    let mut best_crossword: Option<Crossword> = None;
    let mut best_score: f32 = 0.0;

    for _ in 0..5 {
        let crossword = Crossword::generate(10, 10, entries.clone(), & mut DefaultRandomProvider::new());

        let score = crossword_eval::get_crossword_fitness(&crossword);

        println!("Score: {}", score);

        if score > best_score {
            best_score = score;
            best_crossword = Some(crossword);
        }
    }
    
    if let Some(best) = & mut best_crossword {
        crossword_printer::print(&best);

        let missing_words = crossword_eval::get_missing_words(&best);
        println!("Missing words: {:?}", missing_words);

        let _ = fs::write("output/missing_words.txt", missing_words.join("\n") + "\n");

        let mut generated_clues:Vec<Entry> = Vec::new();

        for word in missing_words {
            if let Ok(clue) = clue_fetcher::get_clue(word.clone(), &config) {
                let added = &best.try_add_entry(&clue);
                if !added {
                    println!("Failed to add {}, unexpected how this can happen. To investigate", clue.get_clue());
                }

                generated_clues.push(clue);
            }
        }

        let _ = fs::write("output/index.html", html_generator::generate_crossword_html(&best, &generated_clues, &config) + "\n");

        
    }
    else {
        println!("No crossword could be generated, end");
    }
}

fn get_data(config:&Config) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let mut fetch_from_api: bool = false;

    #[cfg(not(debug_assertions))]
    {
        fetch_from_api = true;
    }

    if fetch_from_api {
        get_api_data(&config)
    }
    else {
        let entries = [
            Entry::new("danes".to_string(), "pred jutri".to_string(), None),
            Entry::new("spore".to_string(), "hyped video game from the '00s".to_string(), None),
            Entry::new("vladar".to_string(), "oblast".to_string(), None),
            Entry::new("prosim".to_string(), "obratno kot hvala".to_string(), None),
            Entry::new("mir".to_string(), "obratno kot vojna".to_string(), None),
            Entry::new("vlak".to_string(), "boljse kot avto in letalo".to_string(), None),
            Entry::new("oblak".to_string(), "pojavi se takrat ko dezuje".to_string(), None),
            Entry::new("luna".to_string(), "polna enkrat vsakih 28 dni".to_string(), None),
            Entry::new("urnik".to_string(), "velika tabela casov".to_string(), None)
        ].to_vec();
        return  Ok(entries);
    }
}

fn get_api_data(config:&Config) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let results: Vec<Entry> = get(&config.clue_get_api)?.json()?;

    let entries:Vec<Entry> = results.into_iter().map(Entry::from).collect();

    Ok(entries)
}
