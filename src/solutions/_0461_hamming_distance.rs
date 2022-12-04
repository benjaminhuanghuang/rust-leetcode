/*
    461. Hamming Distance

    https://leetcode.com/problems/hamming-distance/

    Easy
*/

pub struct Solution;

impl Solution {
  pub fn hamming_distance(x: i32, y: i32) -> i32 {
    (x ^ y).count_ones() as i32
  }

  pub fn hamming_distance2(x: i32, y: i32) -> i32 {
    let mut xor = x ^ y;
    let mut distance = 0;
    while xor != 0 {
      distance += xor & 1;
      xor >>= 1;
    }
    return distance;
  }

  pub fn hamming_distance_fastest(x: i32, y: i32) -> i32 {
    let mut xor = x ^ y;
    let mut distance = 0;
    while xor != 0 {
      distance += 1;
      xor &= xor - 1;
    }
    return distance;
  }

  /*
    https://www.youtube.com/watch?v=2t6bgm7gUoQ&t=58s
  */
  pub fn hamming_distance_str(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
      None
    } else {
      let s1_chars = s1.chars();
      let s2_chars = s2.chars();

      let mut distance: usize = 0;
      for (e1, e2) in s1_chars.zip(s2_chars) {
        if e1 != e2 {
          distance += 1;
        }
      }
      Some(distance)
    }
  }
  pub fn hamming_distance_str2(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
      None
    } else {
      Some(
        s1.chars()
          .zip(s1.chars())
          .map(|(e1, e2)| if e1 != e2 { 1 } else { 0 })
          .sum(),
      )
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_461() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(3, 1), 1);

    assert_eq!(Solution::hamming_distance2(1, 4), 2);
    assert_eq!(Solution::hamming_distance2(3, 1), 1);
  }
}
