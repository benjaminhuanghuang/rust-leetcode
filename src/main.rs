mod bitwise;
mod common;
mod other;
mod string;

fn main() {
  assert_eq!(bitwise::hamming_distance_0461::Solution::hamming_distance(1, 4), 2);
}
