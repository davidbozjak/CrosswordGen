mod crossword;
mod entry;
mod random_provider;
mod crossword_printer;
mod html_generator;

use crossword::Crossword;
use entry::Entry;
use entry::EntryDto;
use random_provider::DefaultRandomProvider;
use std::fs;

use reqwest::blocking::get;

fn main() {
    let entries_apiresult = get_data();

    if entries_apiresult.is_err()
    {
        print!("API error, end");
        return;
    }

    let entries = entries_apiresult.unwrap();

    let crossword = Crossword::generate(10, 10, entries, & mut DefaultRandomProvider::new());

    crossword_printer::print(&crossword);

    let _ = fs::write("output/index.html", html_generator::generate_crossword_html(&crossword));
}

fn get_data() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let fetch_from_api: bool = true;

    #[cfg(not(debug_assertions))]
    {
        fetch_from_api = true;
    }

    if fetch_from_api {
        get_api_data()
    }
    else {
        let entries = [
            Entry::new("danes".to_string(), "pred jutri".to_string()),
            Entry::new("spore".to_string(), "hyped video game from the '00s".to_string()),
            Entry::new("vladar".to_string(), "oblast".to_string()),
            Entry::new("prosim".to_string(), "obratno kot hvala".to_string()),
            Entry::new("mir".to_string(), "obratno kot vojna".to_string()),
            Entry::new("vlak".to_string(), "boljse kot avto in letalo".to_string()),
            Entry::new("oblak".to_string(), "pojavi se takrat ko dezuje".to_string()),
            Entry::new("luna".to_string(), "polna enkrat vsakih 28 dni".to_string()),
            Entry::new("urnik".to_string(), "velika tabela casov".to_string())
        ].to_vec();
        return  Ok(entries);
    }
}

fn get_api_data() -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let results: Vec<EntryDto> = get("https://sheetdb.io/api/v1/2ugi4z3mi964f")?.json()?;

    let entries:Vec<Entry> = results.into_iter().map(Entry::from).collect();

    Ok(entries)
}