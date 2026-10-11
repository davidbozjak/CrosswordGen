mod crossword;
mod entry;
mod random_provider;
mod crossword_printer;
mod html_generator;

use crossword::Crossword;
use entry::Entry;
use random_provider::DefaultRandomProvider;
use std::fs;

use reqwest::blocking::get;
use serde::Deserialize;

fn main() {
    let entries_apiresult = get_data();

    if (entries_apiresult.is_err())
    {
        print!("API error, end");
        return;
    }

    let entries = entries_apiresult.unwrap();

    let crossword = Crossword::generate(entries, & mut DefaultRandomProvider::new());

    crossword_printer::print(&crossword);

    fs::write("output/index.html", html_generator::generate_crossword_html(&crossword));
}

fn get_data() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let get_API_data: bool = true;

    #[cfg(not(debug_assertions))]
    {
        get_API_data = true;
    }

    if get_API_data {
        get_api_data()
    }
    else {
        let entries = [
            Entry::new("danes", "pred jutri"),
            Entry::new("spore", "hyped video game from the '00s"),
            Entry::new("vladar", "oblast"),
            Entry::new("prosim", "obratno kot hvala"),
            Entry::new("mir", "obratno kot vojna"),
            Entry::new("vlak", "boljse kot avto in letalo"),
            Entry::new("oblak", "pojavi se takrat ko dezuje"),
            Entry::new("luna", "polna enkrat vsakih 28 dni"),
            Entry::new("urnik", "velika tabela casov")
        ].to_vec();
        return  Ok(entries);
    }
}

fn get_api_data() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let results: Vec<Entry> = get("https://sheetdb.io/api/v1/2ugi4z3mi964f")?.json()?;

    Ok(results)
}