/*
  5. Longest Palindromic Substring

  https://leetcode.com/problems/longest-palindromic-substring/

  Medium

  # 647. Palindromic Substrings
  # 516. Longest Palindromic Subsequence

*/
pub struct Solution;

/*
  Brute force
  Time complexity: O(n^3).

  for(start){
    for(end){
      isPalinfromic(start, end)
    }
  }
*/

/*
Try all possible i and find the longest palindromic string whose center is i (odd case) and i / i + 1 (even case).

Time complexity: O(n^2)

Space complexity: O(1)
*/
use std::cmp;

impl Solution {
  pub fn get_len(chars: &Vec<char>, mut l: usize, mut r: usize) -> usize {
    // can not use l >=0 for usize type
    while l != std::usize::MAX && r < chars.len() && chars[l] == chars[r] {
      // l -= 1;  DOES not work
      // l = ((l as isize)-1) as usize;
      l = l.checked_sub(1).unwrap_or(chars.len());  
      r += 1;
    }
    //
    // ((r as isize - l as isize) - 1) as usize
    r.checked_sub(l).unwrap_or(chars.len()) - 1
  }

  pub fn longest_palindrome(s: String) -> String {
    let mut max_len = 0;
    let mut start = 0;
    // to access char by index
    let s: Vec<char> = s.chars().collect();

    for i in 0..s.len() {
      let len = cmp::max(Solution::get_len(&s, i, i), Solution::get_len(&s, i, i + 1));
      if len > max_len {
        max_len = len;
        start = i - (len - 1) / 2;
      }
    }

    s[start..start + max_len].iter().collect()
  }

  pub fn longest_palindrome2(s: String) -> String {
    // to access char by index
    let s: Vec<char> = s.chars().collect();
    let len = s.len();
    if len == 0 {
      return "".to_string();
    }

    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
      let mut left = i;
      let mut right = i;
      // use "aa" as the middle
      while right + 1 < len && s[right + 1] == s[left] {
        right += 1;
      }
      // use "a" as the middle
      while right + 1 < len && left > 0 && s[right + 1] == s[left - 1] {
        right += 1;
        left -= 1;
      }

      if right - left > end - start {
        end = right;
        start = left;
      }
    }
    s[start..=end].iter().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    // assert_eq!(
    //   Solution::longest_palindrome(String::from("badad")),
    //   String::from("ada")
    // );
    println!("{}", Solution::longest_palindrome(String::from("ssaasdbc")));
    // assert_eq!(
    //   Solution::longest_palindrome(String::from("ssaasdbc")),
    //   String::from("saas")
    // );
    // assert_eq!(
    //   Solution::longest_palindrome(String::from("tattarrattat")),
    //   String::from("tattarrattat")
    // );
  }
}
