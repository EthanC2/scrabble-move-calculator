# scrabble-move-calculator


# Why this library?
This library uses a trie and combinatorics to lowest memory footprint possible

# Future Plans
- Add support for wildcards
- Add support for non-ASCII Scrabble

# Examples

Creating a dictionary
```rust
//Default wordlist 
let mut default_dict = ScrabbleMoveCalculator::new();

//Load a dictionary from a wordlist (can return io::Result<Self>)
let dict = ScrabbleMoveCalculator::load_from_file("src/smc/twl06.txt")
           .expect("could not find file!");
```

Add words to a dictionary
```rust
//Add a single word to the dictionary
default_dict.add("practice");

//Add a list of words to the dictionary
default_dict.add_from_file("src/smc/twl06.txt")
            .expect("could not find dictionary");
```

Checking the amount of points a word is worth
```rust
let points = ScrabbleMoveCalculator::calculate_points("hello");
println!("hello is worth {} points", points); 
```

Checking if a word is playable
```rust
if dict.contains("hello") {
        println!("hello is legal!");
    } else {
        println!("hello is not legal!");
}
```

Generating all possible plays
```rust
for (word, points) in dict.generate_plays("setamep") {
        println!("{}: {}", points, word);
}
```