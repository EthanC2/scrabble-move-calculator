//Modules
mod combinatorics;
mod trie;

//Standard library
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

//Crates
use lazy_static::lazy_static;

//From this project
use trie::Trie;

//Static lookup table for point values
lazy_static! {
static ref POINTLUT: HashMap<char, u8> = HashMap::from([
    ('a', 1), ('e', 1), ('i', 1), ('o', 1), ('u', 1), ('r', 1), ('s', 1), ('t', 1), ('l', 1), ('n', 1),        //1 point letters
    ('d', 2), ('g', 2),                                                                                       //2 point letters
    ('b', 3), ('c', 3), ('m', 3), ('p', 3),                                                                  //3 point letters
    ('f', 4), ('h', 4), ('v', 4), ('w', 4), ('y', 4),                                                       //4 point letters
    ('k', 5),                                                                                              //5 point letters
    ('j', 8), ('x', 8),                                                                                   //8 point letters
    ('q', 10), ('z', 10),                                                                                //10 point letters
    ('?', 0)    
]);
}


pub struct ScrabbleMoveCalculator {
    dictionary: Trie,
}

impl ScrabbleMoveCalculator {
    //Constructors
    pub fn new() -> Self {
        ScrabbleMoveCalculator{dictionary: Trie::new()}
    }

    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut dict = ScrabbleMoveCalculator::new();
        dict.add_from_file(path)?;
        Ok(dict)
    }

    //Adding to the dictionary
    pub fn add(&mut self, word: &str) {
        //Todo: force lowercase conversion
        self.dictionary.add_word(word);
    }

    pub fn add_from_file(&mut self, path: &str) -> io::Result<()> {
        let f = File::open(path)?;
        let f = BufReader::new(f);

        for line in f.lines() {
            self.add(&line.unwrap());
        }

        Ok(())
    }

    //Generating plays
    pub fn generate_plays(&self, hand: &str, sort: bool) -> Vec<(String, u32)> {
        let mut playable_words = Vec::new();

        for word in combinatorics::permutations(hand) {
            if self.dictionary.contains(&word) {
                let point_value = ScrabbleMoveCalculator::calculate_points(&word);
                playable_words.push( (word, point_value) );
            }
        }

        for word in combinatorics::power_set(hand) {
            if self.dictionary.contains(&word) {
                let point_value = ScrabbleMoveCalculator::calculate_points(&word);
                playable_words.push( (word, point_value) );
            }
        }

        //Sort words by point value, if requested
        if sort {
            playable_words.sort_unstable_by(|a, b| (b.1).partial_cmp(&a.1).unwrap());
        }

        playable_words
    }

    //Misc
    pub fn contains(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    pub fn calculate_points(word: &str) -> u32 {
        //A word is worth the sum of the value of its characters
        word.chars().fold(0, |acc, c| acc + *POINTLUT.get(&c).unwrap_or(&0u8) as u32)
    }   
}