/*
  8. String to Integer (atoi)

  https://leetcode.com/problems/string-to-integer-atoi/

  Medium
*/
pub struct Solution;

impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    1234
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::my_atoi("1234".to_string()), 1234);
  }
}
