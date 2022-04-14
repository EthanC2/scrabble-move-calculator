pub fn power_set(word: &str) -> Vec<String> {
    let len = &word.chars().count();

    //Preallocate a vector with 2^n slots
    let powset_size = 2usize.pow(*len as u32);
    let mut powset = Vec::with_capacity(powset_size);
    let mut buffer = String::with_capacity(*len);

    //Generate the power set of the string
    //CAREFUL: 'for i in 0..powset_size' changed to 'for i in 1..powset_size-1'
    //          skip empty set and prevent overlap with 1st value from permutations
    for i in 1..powset_size-1 {  
        for j in 0..powset_size {
            let lshift = usize::checked_shl(1, j as u32).unwrap_or_default();
            if i & lshift != 0 {    //if i & (1 << j) != 0 {
                buffer.push(word.chars().nth(j).unwrap());
            }
        }
        powset.push(buffer.clone());
        buffer.clear();
    }

    powset
}

pub fn permutations(word: &str) -> Vec<String> {
    //Setup for buffer
    let len = &word.chars().count();
    let mut buffer = String::from(word);

    //Setup for vector to hold all the permutations
    let perms_size = (1..=*len).product();   //condensed factorial
    let mut all_permutations = Vec::with_capacity(perms_size);
    
    permute(&mut buffer, 0, *len-1, &mut all_permutations);    
    all_permutations
}

fn permute(word: &mut String, left: usize, right: usize, all_permutations: &mut Vec<String>) {
    if left == right {
        all_permutations.push(word.clone());
    } else {
        for i in left..=right {
            *word = swap(word, left, i);
            permute(word, left+1, right, all_permutations);
            *word = swap(word, left, i);
        }
    }
}

fn swap(s: &mut String, i: usize, j: usize) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.swap(i, j);
        chars.into_iter().collect()
}