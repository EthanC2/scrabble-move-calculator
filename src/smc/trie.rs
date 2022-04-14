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
            //todo: refactor using '.entry_or(TrieNode::new())'
            if !current.children.contains_key(&c) {
                current.children.insert(c, TrieNode::new());
            }
            
            current = current.children.get_mut(&c).unwrap();
        }

        current.is_word = true;
    }
    
    pub fn contains(&self, word: &str) -> bool {
        return self.internal_search(&word, &self.root);
    }
    
    fn internal_search(&self, word: &str, node: &TrieNode) -> bool {
        let mut current = node;
        
        for (i, c) in word.chars().enumerate() {
            if c == '?' {
                return current.children
                       .values()
                       .any(|node| self.internal_search(&word[(i+1)..], node));
            } else {
                if !node.children.contains_key(&c) {
                    return false;
                }
                
                current = current.children.get(&c).unwrap();
            }
        }
        
        return current.is_word;
    }
}