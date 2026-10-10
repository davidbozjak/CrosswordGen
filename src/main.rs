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

    let crossword = Crossword::generate(&[&entry1, &entry2], & mut DefaultRandomProvider::new());

    crossword_printer::print(crossword);
}
