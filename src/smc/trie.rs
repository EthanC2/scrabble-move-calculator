use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char,TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {children: HashMap::new(), is_word: false}
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {

    pub fn new() -> Self {
        Trie{root: TrieNode::new()}
    }
    
    pub fn add_word(&mut self, word: &str) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current.children.entry(c).or_insert(TrieNode::new());
        }

        current.is_word = true;
    }
    
    pub fn contains(&self, word: &str) -> bool {
        return self.internal_search(&word, &self.root);
    }
    
    fn internal_search(&self, word: &str, node: &TrieNode) -> bool {
        let mut current = node;
        
        for (i, c) in word.chars().enumerate() {
            match c {
                '?' => return current.children
                       .values()
                       .any(|node| self.internal_search(&word[(i+1)..], node)),
                _ => match current.children.get(&c) {
                    Some(child) => current = child,
                    None => return false,
                }
            }
        }
        
        return current.is_word;
    }
}