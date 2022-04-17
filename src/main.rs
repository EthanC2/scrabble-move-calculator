extern crate lazy_static;
mod smc;
use smc::ScrabbleMoveCalculator;


fn main() {
     let dict = ScrabbleMoveCalculator::load_from_file("src/smc/twl06.txt").expect("could not find file!");
    
     for (word, points) in dict.generate_plays("setam") {
         println!("{}: {}", points, word);
     }
}