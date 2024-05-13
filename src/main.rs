use std::collections::HashMap;

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

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: HashMap<u8, Node>,
}

#[derive(Debug, Default)]
struct Trie {
    root: Node,
    len: usize,
}
impl Trie {
    fn new() -> Self {
        Trie::default()
    }
    fn insert(&mut self, text: &str) {
        let mut current_node = &mut self.root;
        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.at_end = true;
    }
    fn contains(&self, text: &str) -> bool {
        let mut current_node = &self.root;
        for c in text.bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.at_end
    }
    fn len(&self) -> usize {
        self.len
    }
}
