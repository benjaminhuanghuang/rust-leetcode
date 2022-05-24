/*
  8. String to Integer (atoi)

  https://leetcode.com/problems/string-to-integer-atoi/

  Medium
*/
pub struct Solution;

impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    let mut chrs = s.chars().skip_while(|c| c == &' ').peekable();
    let mut sign = 1;
    let mut def = std::i32::MAX;
    // process the first char
    match chrs.peek() {
      Some(&'-') => {
        sign = -1;
        def = std::i32::MIN;
        chrs.next();
      }
      Some(&'+') => {
        chrs.next();
      }
      None => {
        return 0;
      }
      _ => {}
    }
    match chrs.peek() {
      Some(&c) => {
        if !c.is_ascii_digit() {
          return 0;
        }
      }
      None => {
        return 0;
      }
    }
    chrs
      .take_while(|c| c.is_ascii_digit())
      .collect::<String>()
      .parse::<i32>()
      .map(|n| n * sign)
      .unwrap_or(def)
  }

  pub fn my_atoi_2(s: String) -> i32 {
    let mut result = 0;
    let mut seen_num = false;
    let mut negative = false;
    for c in s.trim_start().chars() {
      match c {
        '0'..='9' => {
          seen_num = true;
          result = result * 10 + (c as i64 - '0' as i64);
          if negative {
            if result > (i32::max_value() as i64) + 1 {
              return -2147483648;
            }
          } else {
            if result > (i32::max_value() as i64) {
              return 2147483647;
            }
          }
        }
        '-' => {
          if seen_num {
            break;
          } else {
            negative = true;
            seen_num = true;
          }
        }
        '+' => {
          if seen_num {
            break;
          } else {
            seen_num = true;
          }
        }
        _ => break,
      }
    }

    (if negative { -result } else { result }) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::my_atoi("-42".to_string()), -42);
    assert_eq!(Solution::my_atoi("".to_string()), 0);
    assert_eq!(Solution::my_atoi("   4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("words and 4193".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), std::i32::MIN);
    assert_eq!(
      Solution::my_atoi("9223372036854775808".to_string()),
      std::i32::MAX
    );
  }
}
