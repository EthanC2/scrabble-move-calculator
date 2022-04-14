use smc::ScrabbleMoveCalculator;

fn power_set(word: &str) -> Vec<String> {
    let len = &word.chars().count();

    //Preallocate a vector with 2^n slots
    let powset_size = 2usize.pow(*len as u32);
    let mut powset = Vec::with_capacity(powset_size);
    let mut buffer = String::with_capacity(*len);

    //Generate the power set of the string
    //CAREFUL: 'for i in 0..powset_size' changed to 'for i in 1..powset_size-1'
    //          skip empty set and prevent overlap with 1st value from permutations
    for i in 0..powset_size {  
        for j in 0..powset_size {
            if i & (1 << j) != 0 {    //if i & (1 << j) != 0 {
                buffer.push(word.chars().nth(j).unwrap());
            }
        }
        powset.push(buffer.clone());
        buffer.clear();
    }

    powset
}

extern crate lazy_static;

mod smc;

fn main() {
    // let dict = ScrabbleMoveCalculator::load_from_file("src/smc/twl06.txt").expect("could not find file!");
    
    // for (word, points) in dict.generate_plays("setam", true) {
    //     println!("{}: {}", points, word);
    // }

    for s in power_set("setam") {
        println!("{}", s);
    }
}