/*
    720. Longest Word in Dictionary

    https://leetcode.com/problems/longest-word-in-dictionary/

    Medium
*/
use std::collections::HashMap;
#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert(&mut self, word: String, longest: &mut String) {
        let mut node = self;
        let mut is_new_word = true;

        for (i, c) in word.chars().enumerate() {
            node = node.children.entry(c).or_default();
            if i == word.len() - 1 {
                node.is_end = true;
            }
            if node.is_end == false {
                is_new_word = false;
            }
        }

        if is_new_word == false {
            return;
        }
        if longest.len() < word.len() || (longest.len() == word.len() && word < longest.to_string())
        {
            longest.clear();
            longest.extend(word.chars());
        }
    }
}
pub struct Solution;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        // sort the words
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        // create Trie
        let mut trie = TrieNode::default();

        let mut result = String::new();
        for word in words {
            trie.insert(word, &mut result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_720() {}
}
