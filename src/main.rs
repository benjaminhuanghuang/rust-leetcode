mod bitwise;
mod other;
mod string;
mod utils;

fn main() {
  assert_eq!(
    bitwise::hamming_distance_0461::Solution::hamming_distance(1, 4),
    2
  );
}
