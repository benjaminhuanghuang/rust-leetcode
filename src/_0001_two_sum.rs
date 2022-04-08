/*
https://leetcode.com/problems/two-sum/

Easy

*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // value -> index:usize
    let mut hash = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
      let complement = target - *num;
      if hash.contains_key(&complement) {
        return vec![hash[&complement], i as i32];
      }
      hash.insert(num, i as i32);
    }
    vec![]
  }

  pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // value -> index:usize
    let mut hash: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
      let complement = target - num;
      if let Some(&complement_index) = hash.get(&complement) {
        return vec![complement_index as i32, i as i32];
      }
      hash.insert(*num, i);
    }
    vec![]
  }

  pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num1) in nums.iter().enumerate() {
      //enumerate() starts from 0
      for (j, num2) in nums.iter().skip(i + 1).enumerate() {
        if num1 + num2 == target {
          return vec![i as i32, (j + 1 + i) as i32];
        }
      }
    }
    vec![]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }

  #[test]
  fn test_2_success() {
    assert_eq!(Solution::two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }

  #[test]
  fn test_naive_success() {
    assert_eq!(Solution::two_sum_naive(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }
}
