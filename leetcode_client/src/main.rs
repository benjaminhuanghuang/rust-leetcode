use std::env::{args, Args};

const LEETCODE_URL = "https://leetcode.com"
const LEETCODE_PROBLEMS_BASE_URL = "https://leetcode.com/problems/"

/* 
  LEETCODE_PROBLEMS_API_URL returns all problem list 
  [
    {
      question_id, 
      quesiton_title
      ...
    }
  ]
*/
const LEETCODE_PROBLEMS_API_URL = "https://leetcode.com/api/problems/algorithms/"

/*

*/
const LEETCODE_GRPAHQL_API_URL = "https://leetcode.com/graphql"

const ROOT_PATH = "/Users/bhuang/ben-github/rust-leetcode/"

fn main() {
  let mut args: Args = args();
  let problem_number = args.nth(1).unwrap();


}


fn downloadProblem() -> String{

} 


fn generateSoluction(&str problem) {

}