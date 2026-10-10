use crate::crossword::Crossword;

pub fn print(crossword: Crossword) {
    
    println!("Created crossword");
    println!("");
    println!("");

    let min_x = crossword.min_x();
    let max_x = crossword.max_x() + 1;
    let min_y = crossword.min_y();
    let max_y = crossword.max_y() + 1;

    for x in min_x .. max_x {
        for y in min_y .. max_y {
            match crossword.get_char_at(x, y) {
                Some(w) => print!("{}", w),
                None => print!(" ")
            };
        }
        println!("");
    }
}