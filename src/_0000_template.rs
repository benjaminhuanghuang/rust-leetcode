/*
https://leetcode.com/problems/two-sum/

Easy

*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn do_something(nums: Vec<i32>, target: i32) -> Vec<i32> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(Solution::do_something(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
