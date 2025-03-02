/*
  461. Hamming Distance

  https://leetcode.com/problems/hamming-distance/

  Easy
*/

pub struct Solution;

/*
Hamming distance is a measure of the difference between two strings of equal length.
It is defined as the number of positions at which the corresponding symbols differ.

The Hamming distance is equal to the number of 1s in a XOR b.
*/
impl Solution {
  /*
  Count the number of 1s in the XOR result of two numbers.
  */
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
