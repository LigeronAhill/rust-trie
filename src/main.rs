use rust_trie::Trie;

fn main() {
    let mut trie = Trie::new();
    dbg!(&trie);
    trie.insert("https://www.reddit/r/rust");
    trie.insert("https://www.google.com");
    let contains = trie.contains("https://www.google.com");
    dbg!(contains);
    let not_contains = trie.contains("https://ya.ru");
    dbg!(not_contains);
    let not_contains_2 = trie.contains("https://www");
    dbg!(not_contains_2);
    dbg!(trie.len());
}
