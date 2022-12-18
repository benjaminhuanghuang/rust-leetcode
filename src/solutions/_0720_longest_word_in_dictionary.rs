/*
    720. Longest Word in Dictionary

    https://leetcode.com/problems/longest-word-in-dictionary/

    Medium
*/
use std::collections::HashMap;

pub struct Solution;

struct TrieNode {
    value: Option<char>,
    is_end: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    pub fn view(c: char, is_end: bool) -> Self {
        Self {
            value: Option::Some(c),
            is_end,
            children: HashMap::new(),
        }
    }

    pub fn check(&self, c: char) -> bool {}
}
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_720() {}
}
