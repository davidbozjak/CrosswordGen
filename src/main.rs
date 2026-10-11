mod crossword;
mod entry;
mod random_provider;
mod crossword_printer;
mod html_generator;

use crossword::Crossword;
use entry::Entry;
use random_provider::DefaultRandomProvider;
use std::fs;


fn main() {
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
    ];

    let crossword = Crossword::generate(entries, & mut DefaultRandomProvider::new());

    crossword_printer::print(&crossword);

    fs::write("output/index.html", html_generator::generate_crossword_html(&crossword));
}
