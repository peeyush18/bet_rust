use radix_trie::Trie;
use std::collections::HashMap;

pub fn init_trie() -> Trie<String, i32> {
    return Trie::new();
}

pub fn add_word_to_trie(word: String, line_num: i32, store: &mut Trie<String, i32>) {
    store.insert(word, line_num);
}

pub fn search_word_line_num(word: &String, store: & Trie<String, i32>) -> Option<i32> {
    return store.get(word).copied()
}

pub fn init_hash_map() -> HashMap<String, i32> {
    return HashMap::new();
}

pub fn add_word_to_hash_map(word: String, line_num: i32, store: &mut HashMap<String, i32>) {
    store.insert(word, line_num);
}

pub fn search_fast_word_line_num(word: &String, store: & HashMap<String, i32>) -> Option<i32> {
    return store.get(word).copied()
}
