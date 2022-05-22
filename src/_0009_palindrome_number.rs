/*
  9. Palindrome Number

  https://leetcode.com/problems/palindrome-number/

  Easy
*/
pub struct Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false;
    }

    let mut origin = x;
    let mut reversed = 0;
    while origin > 0 {
      reversed = reversed * 10 + origin % 10;
      origin /= 10;
    }

    x == reversed
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::do_something(), vec![0]);
  }
}
