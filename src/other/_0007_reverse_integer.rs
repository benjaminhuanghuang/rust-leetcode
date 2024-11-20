/*
  7. Reverse Integer

  https://leetcode.com/problems/reverse-integer/

  Medium
*/
use std::convert::TryFrom;

pub struct Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let mut reversed = 0;
    let mut num = x as i64;

    while num.abs() > 0 {
      reversed *= 10;
      reversed += num % 10;
      num /= 10;
    }
    // prevent overflow
    // return reversed;
    i32::try_from(reversed).unwrap_or(0)
  }

  /*
    https://www.youtube.com/watch?v=556jh2RJM1I&ab_channel=ZaneHitchcox
  */
  pub fn reverse_str(x: i32) -> i32 {
    x.signum()
      * x
        .abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::reverse(2), 2);
  }
}
