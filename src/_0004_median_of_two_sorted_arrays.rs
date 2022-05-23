/*
  4. Median of Two Sorted Arrays

  https://leetcode.com/problems/median-of-two-sorted-arrays/

  Hard
*/
pub struct Solution;

/*
  Easy solution:
  is Merging two arrays then find median, the time complexity of merge two arrays is O(M+N)
  The time complexity will be O(N+M)

*/
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
  let mut all = vec![];
  let (mut i, mut j) = (0, 0);
  let total_len = nums1.len() + nums2.len();

  for _ in 0..total_len {
    // nums1[i] will cause error: the len is 0 but the index is 0
    let num1 = nums1.get(i).or(Some(&std::i32::MAX)).unwrap(); // num1 is a option
    let num2 = nums2.get(j).or(Some(&std::i32::MAX)).unwrap();

    if num1 < num2 {
      i += 1;
      all.push(*num1);
    } else {
      j += 1;
      all.push(*num2);
    }
  }

  let median = total_len / 2;
  if total_len % 2 == 0 {
    return (all[median - 1] + all[median]) as f64 / 2.0;
  }
  all[median] as f64
}

/*
  Binary search
  https://www.youtube.com/watch?v=q6IEA26hvXc&ab_channel=NeetCode

  根据元素总个数，可以知道最终left half, right half各有多少元素
  1. 对一个array 二分，根据二分后left part 的长度和最终 left half的长度，可求出另一array中left part 的最后一个元素和right part的第一个元素
     如果 每组的last of left < first of right，就找到了答案
  3. 先指定一个array, l=0, r=length, 用 (l+r)/2 来划分left partition， right partition， 那么在另一array中，right partition要比这个值大
     如果不对，就调整这个array的l,r 边界

  4. max(两个左partition) + min(两个右partition)

  Time O(log(min(N,M))
*/
impl Solution {
  fn find_middle(nums: Vec<i32>) -> f64 {
    let len = nums.len();
    if len == 0 {
      panic!()
    }
    let mid = len / 2;
    if len % 2 == 0 {
      (nums[mid] + nums[mid - 1]) as f64 / 2.0
    } else {
      nums[mid] as f64
    }
  }
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() {
      return Self::find_middle(nums2);
    }
    if nums2.is_empty() {
      return Self::find_middle(nums1);
    }
    let total = nums1.len() + nums2.len();
    let half = total as i32 / 2;

    // step1: find the shorter ones that
    let (mut a, mut b) = (&nums1, &nums2);
    if nums1.len() > nums2.len() {
      b = &nums1;
      a = &nums2;
    }
    // step 2. run binary search on the shorter array
    let mut l: i32 = 0;
    let mut r: i32 = a.len() as i32 - 1; // Break when a.len() is 0

    loop {
      // when r = -1, l = 0, I want to get -1, but (r - l) / 2 = 0
      let mid: i32 = (((r - l) as f32 / 2.0).floor() + l as f32) as i32; // for A
      let mid_b: i32 = half - mid - 2; // the middle index for B

      // check out of bounds
      let left_a = a.get(mid as usize).or(Some(&std::i32::MIN)).unwrap();
      let right_a = a.get((mid + 1) as usize).or(Some(&std::i32::MAX)).unwrap();

      let left_b = b.get(mid_b as usize).or(Some(&std::i32::MIN)).unwrap();
      let right_b = b
        .get((mid_b + 1) as usize)
        .or(Some(&std::i32::MAX))
        .unwrap();

      if *left_a <= *right_b && *left_b <= *right_a {
        // find it
        if total % 2 == 1 {
          // odd
          return std::cmp::min(*right_a, *right_b) as f64;
        } else {
          return (std::cmp::max(*left_a, *left_b) + std::cmp::min(*right_a, *right_b)) as f64
            / 2.0;
        }
      } else if *left_a > *right_b {
        r = mid - 1;
      } else {
        l = mid + 1;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(
      Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
      2.0
    );
  }

  #[test]
  fn test_success2() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
  }
}
