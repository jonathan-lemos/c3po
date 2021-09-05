use std::collections::HashMap;

pub struct LiteralTree {
    member: bool,
    children: HashMap<char, LiteralTree>
}

impl LiteralTree {
    pub fn new() -> Self {
        LiteralTree {
            member: false,
            children: HashMap::new()
        }
    }

    fn get_child(&mut self, c: char) -> &mut Self {
        self.children.entry(c).or_insert(LiteralTree::new())
    }

    fn insert_chars<I: Iterator<Item = char>>(&mut self, iter: &mut I) -> &mut Self {
        match iter.next() {
            None => {
                self.member = true;
                self
            },
            Some(c) => {
                self.get_child(c).insert_chars(iter)
            }
        }
    }

    fn contains_chars<I: Iterator<Item = char>>(&self, it: &mut I) -> bool {
        match it.next() {
            None => self.member,
            Some(c) => {
                match self.children.get(&c) {
                    None => false,
                    Some(child) => child.contains_chars(it)
                }
            }
        }
    }

    pub fn contains(&self, s: &str) -> bool {
        self.contains_chars(&mut s.chars())
    }

    pub fn 

    pub fn insert(&mut self, s: &str) -> &mut Self {
        let chars = s.chars();
        self.insert_chars(&mut s.chars())
    }
}
