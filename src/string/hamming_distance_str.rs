pub struct Solution;

/*
  Hamming Distance between two string
  https://www.youtube.com/watch?v=2t6bgm7gUoQ&t=58s
*/
impl Solution {
  pub fn hamming_distance_str(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
      None
    } else {
      let s1_chars = s1.chars();
      let s2_chars = s2.chars();

      let mut distance: usize = 0;

      //.zip() method combines two iterators into a single iterator of pairs (tuples),
      // Iteration stops when the shorter iterator is exhausted.
      for (e1, e2) in s1_chars.zip(s2_chars) {
        if e1 != e2 {
          distance += 1;
        }
      }
      Some(distance)
    }
  }
}
pub struct Solution2;
impl Solution2 {
  pub fn hamming_distance_str(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
      None
    } else {
      Some(
        s1.chars()
          .zip(s2.chars())
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
  fn test_solution1() {
    // Basic test cases
    assert_eq!(Solution::hamming_distance_str("karolin", "kathrin"), Some(3));
    assert_eq!(Solution::hamming_distance_str("1011101", "1001001"), Some(2));
    assert_eq!(Solution::hamming_distance_str("rust", "rust"), Some(0));

    // Different lengths should return None
    assert_eq!(Solution::hamming_distance_str("hello", "hell"), None);
    assert_eq!(Solution::hamming_distance_str("short", "longer"), None);

    // Empty strings
    assert_eq!(Solution::hamming_distance_str("", ""), Some(0));
  }
}
