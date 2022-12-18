use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    pub children: Vec<TrieNode>,
    pub key: Option<char>,
    pub value: Option<String>,
    pub count: usize,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_key(c: char) -> Self {
        Self {
            key: Some(c),
            ..Default::default()
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, s: &str) {
        let mut cur = &mut self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &mut cur.children[i];
                }
                Err(i) => {
                    cur.children.insert(i, TrieNode::with_key(c));
                    cur = &mut cur.children[i];
                }
            }
        }
        cur.value.replace(s.to_string());
    }

    pub fn exists(&mut self, s: &str) -> bool {
        let mut cur = &mut self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &mut cur.children[i];
                }
                Err(i) => {
                    return false;
                }
            }
        }
        cur.count > 0
    }

    pub fn search(&mut self, s: &str) -> Vec<String> {
        let mut cur = &mut self.root;
        for c in s.chars() {
            match cur.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    cur = &mut cur.children[i];
                }
                Err(i) => {
                    return Vec::new();
                }
            }
        }
        let mut results = Vec::new();
        let mut q = Vec::new();
        q.push(cur);
        while let Some(c) = q.pop() {
            for child in c.children.iter_mut() {
                q.push(child);
            }

            if c.count > 0 {
                let value = c.value.as_ref().unwrap();
                let count = c.count;
                results.push((count, value));
            }
        }
        results.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
        results.iter().map(|m| m.1.clone()).collect()
    }
}
