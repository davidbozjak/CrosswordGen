mod crossword;
mod entry;
mod random_provider;
mod crossword_printer;

use crossword::Crossword;
use entry::Entry;
use random_provider::DefaultRandomProvider;

fn main() {
    let entry1 = Entry::new("danes", "pred jutri");
    let entry2 = Entry::new("spore", "hyped video game from the '00s");
    let entry3 = Entry::new("vladar", "oblast");
    let entry4 = Entry::new("prosim", "obratno kot hvala");
    let entry4 = Entry::new("mir", "obratno kot vojna");
    let entry4 = Entry::new("vlak", "boljse kot avto in letalo");

    let crossword = Crossword::generate(&[&entry1, &entry2, &entry3, &entry4], & mut DefaultRandomProvider::new());

    crossword_printer::print(crossword);
}
