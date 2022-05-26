extern crate lazy_static;
mod smc;
use smc::ScrabbleMoveCalculator;


fn main() {
    let calc = ScrabbleMoveCalculator::load_from_file("twl06.txt");
    for (word, points) in calc.generate_plays("dummyhand") {
        println!("{}: {}", word, points);  
    }
}
