mod crossword;
mod entry;
mod random_provider;

use crossword::Crossword;
use entry::Entry;
use random_provider::DefaultRandomProvider;

fn main() {
    let entry1 = Entry::new("danes", "pred jutri");
    let entry2 = Entry::new("spore", "hyped video game from the '00s");

    let crossword = Crossword::Generate(&[&entry1, &entry2], & mut DefaultRandomProvider::new());

    println!("{:?}", entry2);
}
