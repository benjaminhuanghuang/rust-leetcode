/*
  3. Longest Substring Without Repeating Characters
  https://leetcode.com/problems/longest-substring-without-repeating-characters/
*/

use std::collections::HashMap;

pub struct Solution;

/*

Solution: HashTable + Sliding Window
Using a hashtable to remember the last index of every char.
And keep tracking the starting point of a valid substring.

start = max(start, last[s[i]] + 1)
ans = max(ans, i – start + 1)

Time complexity: O(n)
Space complexity: O(128)
*/
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut hash: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    for (i, ch) in s.chars().enumerate() {
      if let Some(&j) = hash.get(&ch) {
        //set start to max(start, j + 1)
        if j + 1 >= start {
          start = j + 1;
        }
      }
      hash.insert(ch, i);
      longest = longest.max(i - start + 1);
    }
    longest as i32
  }

  pub fn length_of_longest_substring2(s: String) -> i32 {
    let mut longest = 0;
    let mut hash: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    for (i, ch) in s.chars().enumerate() {
      if let Some(&j) = hash.get(&ch) {
        if j >= start {
          longest = longest.max(i - start);
          start = j + 1;
        }
      }
      hash.insert(ch, i);
    }
    longest.max(s.len() - start) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(
      Solution::length_of_longest_substring(String::from("abcabcbb")),
      3
    );
    assert_eq!(
      Solution::length_of_longest_substring(String::from("abba")),
      2
    );
    assert_eq!(
      Solution::length_of_longest_substring(String::from("cdd")),
      2
    );
    assert_eq!(
      Solution::length_of_longest_substring(String::from("aab")),
      2
    );

    assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
  }
}
