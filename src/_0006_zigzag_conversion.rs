/*
  6. Zigzag Conversion

  https://leetcode.com/problems/zigzag-conversion/

  Medium
*/
pub struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows < 2 {
      return s;
    }
    let num_rows = num_rows as usize;
    let mut result = vec![String::new(); num_rows];
    let mut curr_row =0;
    let mut down = true;

    for c in s.chars() {
      result[curr_row] .push(c);
      curr_row = if down {curr_row + 1} else {curr_row-1};

      if down && curr_row >= num_rows -1 {
        down = false;
      }
      if !down && curr_row <= 0 {
        down = true;
      }
    }

    // Flattens a slice of T into a single value
    result.concat()
  }

  pub fn convert2(s: String, num_rows: i32) -> String {
    let n = s.len() as i32;
    if num_rows < 2 {
      return s;
    }
    let sv: Vec<char> = s.chars().collect();
    let mut result = "".to_string();
    let cycle_len = num_rows * 2 - 2;

    for row in 0..num_rows {
      let mut cycle = 0;
      while cycle + row < n {
        result.push(sv[(cycle + row) as usize]);
        if row != 0 && row != num_rows - 1 && cycle + cycle_len - row < n {
          result.push(sv[(cycle + cycle_len - row) as usize]);
        }
        cycle += cycle_len;
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(
      String::from("Aaidoeswr,haenme,rtesqecouishtabrateaeaietedrcinwtgnrlloacsoajsmnsoucutoadodiiesplnrmiaodprs,ubroohreunefnttacneedhsmwynihrieto,iheeaalwnefrdutettpntainnwrdvdr."),
      Solution::convert("Apalindromeisaword,phrase,number,orothersequenceofunitsthatcanbereadthesamewayineitherdirection,withgeneralallowancesforadjustmentstopunctuationandworddividers.".to_string(), 2),
    );
    assert_eq!(
      String::from("PYAIHRNAPLSIIG"),
      Solution::convert("PAYPALISHIRING".to_string(), 2),
    );
    assert_eq!(String::from("ABC"), Solution::convert("ABC".to_string(), 3),);
    assert_eq!(
      String::from("PAHNAPLSIIGYIR"),
      Solution::convert("PAYPALISHIRING".to_string(), 3),
    );
    assert_eq!(
      String::from("PINALSIGYAHRPI"),
      Solution::convert("PAYPALISHIRING".to_string(), 4),
    );
    assert_eq!(String::from("AB"), Solution::convert("AB".to_string(), 1),);
  }
}
