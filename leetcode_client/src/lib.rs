mod entities;

use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::env::{args, Args};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

const LEETCODE_URL: &str = "https://leetcode.com";
const LEETCODE_PROBLEMS_BASE_URL: &str = "https://leetcode.com/problems/";
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

*/
const LEETCODE_GRPAHQL_API_URL: &str = "https://leetcode.com/graphql";

const ROOT_PATH: &str = "/Users/bhuang/ben-github/rust-leetcode/";

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
  let resp_text = reqwest::get(LEETCODE_PROBLEMS_API_URL)
    .expect("")
    .text()
    .expect("");

  // println!("{}", resp_text);
  // Save json response to "leetcode_problems.json"
  let mut f = File::create("leetcode_problems.json").unwrap();
  f.write_all(resp_text.as_bytes()).unwrap();
}

/*
GetProblemDetailByFrontendId calls Leetcode GraphQL API https://leetcode.com/graphql to get problem detail by title slug
*/

fn generate_solution(config: &Config, id: &str) {
  // find detail in the json file_name
  let json_file = std::fs::File::open("leetcode_problems.json").expect("read file error!");
  let leetcode: entities::Leetcode = serde_json::from_reader(json_file).unwrap();
  let front_id: u32 = id.parse().unwrap();
  let target = leetcode
    .stat_status_pairs
    .iter()
    .find(|x| x.stat.frontend_question_id == front_id);

  match target {
    Some(state_status_pair) => {
      let question_title = &state_status_pair.stat.question__title;
      let question_title_slug = &state_status_pair.stat.question__title_slug;
      let question_difficulty = entities::Level::from_u32(state_status_pair.difficulty.level);

      let file_header = format!(
        r#"/*
{}. {}

https://leetcode.com/problems/{}/

{}
*/
pub struct Solution;

impl Solution {{
  pub fn implementation() {{

  }}
}}"#,
        id, question_title, question_title_slug, question_difficulty
      );
      let full_path = format!(
        "{}/_{:0>4}_{}.rs",
        &config.output_folder,
        id,
        question_title_slug.replace("-", "_")
      );
      //println!("{}", full_path);
      let mut f = File::create(full_path).unwrap();
      f.write_all(file_header.as_bytes()).unwrap();
    }
    None => {
      println!("Can not find problem #{}", id);
    }
  }
}
