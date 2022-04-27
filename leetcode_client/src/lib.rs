mod entities;

use clap::{Arg, Command};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

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
const LEETCODE_PROBLEMS_API_URL: &str = "https://leetcode.com/api/problems/algorithms/";
/*
    graphql returns question data
*/
const LEETCODE_GRPAHQL_API_URL: &str = "https://leetcode.com/graphql";

type MyResult<T> = Result<T, Box<dyn Error>>;

/*
  describes the names and types of the arguments
*/
#[derive(Debug)]
pub struct Config {
  question_ids: Vec<String>,
  reload: bool,
  output_folder: String,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("LeetcodeClient")
    .version("0.1.0")
    .author("Benjamin <benjamin@gmail.com>")
    .about("Rust Leetcode client")
    .arg(
      Arg::new("ids")
        .value_name("IDS")
        .help("Input question id(s)")
        .allow_invalid_utf8(true)
        .multiple_values(true),
    )
    .arg(
      Arg::new("reload")
        .short('r')
        .long("reload")
        .help("reload quesions data")
        .takes_value(false),
    )
    .arg(
      Arg::new("output_folder")
        .short('o')
        .long("output-folder")
        .help("Output folder")
        .value_name("OUTPUT_FOLDER")
        .default_value("../src"),
    )
    .get_matches();

  Ok(Config {
    question_ids: matches.values_of_lossy("ids").unwrap(),
    reload: matches.is_present("reload"),
    output_folder: matches.value_of("output_folder").unwrap().to_string(),
  })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
  //println!("..... The config: {:?}", config);
  if config.reload {
    download_questions_data();
  }

  for id in config.question_ids.iter() {
    generate_solution(&config, &id);
  }
  Ok(())
}

/*

*/
fn download_questions_data() {
  let resp_text = reqwest::blocking::get(LEETCODE_PROBLEMS_API_URL)
    .unwrap()
    .text()
    .unwrap();

  // println!("{}", resp_text);
  // Save json response to "leetcode_problems.json"
  let mut f = File::create("leetcode_problems.json").unwrap();
  f.write_all(resp_text.as_bytes()).unwrap();
}

/*
GetProblemDetailByFrontendId calls Leetcode GraphQL API https://leetcode.com/graphql to get problem detail by title slug
*/
fn query_question_data(title_slug: &str) -> String {
  let payload = format!(
    r#"{{
		"operationName": "questionData",
		"variables": {{
			"titleSlug": "{}"
		}},
		"query": "query questionData($titleSlug: String!) {{
      question(titleSlug: $titleSlug) {{
        questionId  
        questionFrontendId 
        title 
        titleSlug 
        difficulty 
        content 
        codeSnippets{{      
          lang      
          langSlug   
          code   
        }}
      }}
    }}"
	}}"#,
    title_slug
  );
  println!("----payload    {:#?}", payload);
  let client = reqwest::blocking::Client::new();

  let res = client
    .post(LEETCODE_GRPAHQL_API_URL)
    .body(payload)
    .send()
    .unwrap();

  println!("----res    {:#?}", res);

  //res
  "".to_string()
}

fn generate_solution(config: &Config, id: &str) {
  // find detail in the json file_name
  let json_file = std::fs::File::open("leetcode_problems.json").expect("read file error!");
  let leetcode: entities::Leetcode = serde_json::from_reader(json_file).unwrap();
  let front_id: u32 = id.parse().unwrap();
  let state_status_pair = leetcode
    .stat_status_pairs
    .iter()
    .find(|x| x.stat.frontend_question_id == front_id)
    .unwrap();

  let question_title = &state_status_pair.stat.question__title;
  let question_title_slug = &state_status_pair.stat.question__title_slug;
  let question_difficulty = entities::Level::from_u32(state_status_pair.difficulty.level);
  // query code snippet  >>> CSRF verification failed
  // let response = query_question_data(&question_title_slug);
  // let data: entities::QuesitonData = serde_json::from_str(&response).unwrap();
  // let code_snippet = data
  //   .question
  //   .code_snippets
  //   .iter()
  //   .find(|x| x.lanSlug == "rust")
  //   .unwrap();
  let code_snippet = r#"
impl Solution {
  pub fn do_something(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    
  }
}
  "#;

  let test_code_snippet = r#"
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_success() {
    assert_eq!(Solution::do_something(), vec![0]);
  }
} 
  "#;

  let solution_content = format!(
    r#"/*
  {}. {}

  https://leetcode.com/problems/{}/

  {}
*/
pub struct Solution;
{}
{}
"#,
    id, question_title, question_title_slug, question_difficulty, code_snippet, test_code_snippet
  );

  let full_path = format!(
    "{}/_{:0>4}_{}.rs",
    &config.output_folder,
    id,
    question_title_slug.replace("-", "_")
  );
  //println!("full_path: {}", full_path);
  let mut f = File::create(full_path).unwrap();
  f.write_all(solution_content.as_bytes()).unwrap();

  // add "pub mod" to lib.rs
  let mod_line = format!(
    "pub mod _{:0>4}_{};",
    id,
    question_title_slug.replace("-", "_")
  );
  add_mod(&mod_line);
}

// append the new solution to lib.rs
fn add_mod(mod_line: &str) {
  let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .open("../src/lib.rs")
    .unwrap();
  file.write_all(mod_line.as_bytes());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_query_question_data() {
    let data = query_question_data("best-time-to-buy-and-sell-stock");
    println!("Question Data: {}", data)
  }

  #[test]
  fn test_bad() {
    assert_eq!(bad_add(1, 2), 3);
  }
}
