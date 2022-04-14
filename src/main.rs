#[macro_use]
extern crate lazy_static;

mod smc;

fn main() {
    let mut dict = smc::ScrabbleMoveCalculator::new();
    dict.add_from_file("src/smc/twl06.txt").expect("could not open dictionary");
}
