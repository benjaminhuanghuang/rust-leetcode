/*
    409. Longest Palindrome

    https://leetcode.com/problems/longest-palindrome/

    Easy
*/
pub struct Solution;
/*
  Solution:
  用hashtable记录每个字符出现的次数，出现偶数次的字符和一个出现奇数次的字符可以构成Palindrome
*/
impl Solution {
  pub fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;

    let mut hm = HashMap::new();
    for ch in s.chars() {
      *hm.entry(ch).or_insert(0) += 1;
    }

    let mut max_len = 0;
    let mut has_odd = false;

    for val in hm.values() {
      if val % 2 == 0 {
        max_len += val;
      } else {
        max_len += val - 1;
        has_odd = true;
      }
    }
    max_len + if has_odd { 1 } else { 0 }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_409() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
  }
}
