extern crate reqwest;

use std::io::prelude::*;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::env::{args, Args};
use serde_derive::Deserialize;
use serde_derive::Serialize;


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
            Arg::with_name("ids")
                .value_name("IDS")
                .help("Input question id(s)")
                .allow_invalid_utf8(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("reload")
                .short('r')
                .long("reload")
                .help("reload quesions data")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("output_folder")
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
    println!("..... The config: {:?}", config);
    if config.reload {
        reload_questions_data();
    }

    for id in config.question_ids.iter() {
        generate_solution(&config, &id);
    }
    Ok(())
}

/*

*/
fn reload_questions_data() {
    let resp_text = reqwest::get(LEETCODE_PROBLEMS_API_URL)
        .expect("")
        .text()
        .expect("");

    //println!("{}", resp_text);
    let mut f = File::create("leetcode_problems.json").unwrap();
    f.write_all(resp_text.as_bytes()).unwrap();
}


/*
GetProblemDetailByFrontendId calls Leetcode GraphQL API https://leetcode.com/graphql to get problem detail by title slug
*/
fn get_problem_detail(frontendId: &str) -> std::io::Result<()> {


fn generate_solution(config: &Config, id: &str) -> std::io::Result<()> {
    let file_name = format!("_{:>04}.rs", id);
    let full_path = config.output_folder.clone() + "/" + &file_name;
    //println!("full_path is {}", full_path);

    let mut f = File::create(full_path).unwrap();
    f.write_all(resp_text.as_bytes()).unwrap();

    //file.write("COVER")?;

    Ok(())
}
