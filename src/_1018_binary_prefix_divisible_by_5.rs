/*
  1018. Binary Prefix Divisible By 5

  https://leetcode.com/problems/binary-prefix-divisible-by-5/

  Easy
*/
pub struct Solution;

impl Solution {
  pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    nums.iter().scan(0, |sum, &x| {*sum = (*sum * 2 + x) % 5;Some(*sum == 0)}).collect()
  }
}
  
/* cpp
vector<bool> prefixesDivBy5(vector<int>& A) {
  int temp = 0;
  vector<bool> res(A.size(), false);
  for (int i = 0; i < A.size(); i++) {
      temp = (temp * 2 + A[i]) % 5;
      if (temp == 0) {
          res[i] = true;
      }
  }
  return res;
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::prefixes_div_by5(vec![0,1,1]), vec![true,false,false]);
  }
} 
  
