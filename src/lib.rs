use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Node {
    pub at_end: bool,
    pub children: HashMap<u8, Node>,
}

#[derive(Debug, Default)]
pub struct Trie {
    pub root: Node,
    pub len: usize,
}
impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }
    pub fn insert(&mut self, text: &str) {
        let mut current_node = &mut self.root;
        for c in text.bytes() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.at_end = true;
        self.len += 1;
    }
    pub fn contains(&self, text: &str) -> bool {
        let mut current_node = &self.root;
        for c in text.bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.at_end
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
