use clap::{Arg, Command};
// use serde_json::Value;
use std::error::Error;
// use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Config {
  question_ids: Vec<String>,
}
type MyResult<T> = Result<T, Box<dyn Error>>;

// const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";
// const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
// const QUESTION_QUERY_STRING: &str = r#"
// query questionData($titleSlug: String!) {
//     question(titleSlug: $titleSlug) {
//         content
//         stats
//         codeDefinition
//         sampleTestCase
//         metaData
//     }
// }"#;
// const QUESTION_QUERY_OPERATION: &str = "questionData";

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("LeetcodeClient")
    .version("0.1.0")
    .author("Ben <ben@gmail.com>")
    .about("Rust Leetcode fetcher")
    .arg(
      Arg::new("ids")
        .value_name("IDS")
        .help("Input question id(s)")
        .allow_invalid_utf8(true)
        .multiple_values(true),
    )
    .get_matches();

  Ok(Config {
    question_ids: matches.values_of_lossy("ids").unwrap(),
  })
}


pub fn run(config: Config) -> MyResult<()> {
    for id in config.question_ids.iter() {

      //generate_solution(&config, &id);
      println!("Fetch problem {}",id);

    }
    Ok(())
  }