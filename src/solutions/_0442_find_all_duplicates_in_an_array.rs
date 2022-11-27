/*
    442. Find All Duplicates in an Array

    https://leetcode.com/problems/find-all-duplicates-in-an-array/

    Medium

    You must write an algorithm that runs in O(n) time and uses only constant extra space.
*/

pub struct Solution;

impl Solution {
  /*
    https://www.youtube.com/watch?v=U6IOcTEvl_M
    Use hashmap and HashSet
  */
  pub fn find_duplicates2(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};

    if nums.len() == 0 {
      return vec![];
    }

    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut result: HashSet<i32> = HashSet::new();

    for i in nums {
      let v = hm.entry(i).or_insert(0);
      *v += 1;
      if *v > 1 {
        result.insert(i);
      }
    }
    result.into_iter().collect()
  }
  /*
  对于每个nums[i]，将其对应的nums[nums[i] - 1]取相反数，如果其已经是负数了，说明之前存在过，将其加入结果res中
  注意要把参数改成 mut
  */
  pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    if nums.len() == 0 {
      return result;
    }

    for i in 0..nums.len() {
      let idx = (nums[i].abs() - 1) as usize;
      if nums[idx] < 0 {
        result.push(idx as i32 + 1);
      }
      nums[idx] = -nums[idx];
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_442() {
    assert_eq!(
      Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
      vec![2, 3]
    );

    assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1])
  }
}
