//use leetcode::_0001_two_sum::Solution;
use leetcode::solutions::_0004_median_of_two_sorted_arrays::Solution;
use leetcode::util::list_node::*;

fn main() {
  println!(
    "{}",
    Solution::find_median_sorted_arrays(vec![2, 3, 4, 5, 6], vec![1])
  );
}
