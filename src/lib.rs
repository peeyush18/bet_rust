use radix_trie::Trie;

pub fn init_trie() -> Trie<String, i32> {
    return Trie::new();
}

pub fn add_word_to_trie(word: String, line_num: i32, store: &mut Trie<String, i32>) {
    store.insert(word, line_num);
}

pub fn search_word_line_num(word: &String, store: & Trie<String, i32>) -> Option<i32> {
    return store.get(word).copied()
}
