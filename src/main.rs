use std::collections::HashMap;

fn main() {
    let mut trie = Trie::new();
    dbg!(&trie);
}

#[derive(Debug, Default)]
struct Node {
    at_end: bool,
    children: HashMap<char, Node>,
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
}
