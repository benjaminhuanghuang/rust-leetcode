extern crate reqwest;

use clap::{Command, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::env::{args, Args};

const LEETCODE_URL:&str = "https://leetcode.com";
const LEETCODE_PROBLEMS_BASE_URL:&str = "https://leetcode.com/problems/";
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
const LEETCODE_PROBLEMS_API_URL:&str = "https://leetcode.com/api/problems/algorithms/";
/*

*/
const LEETCODE_GRPAHQL_API_URL:&str = "https://leetcode.com/graphql";

const ROOT_PATH:&str = "/Users/bhuang/ben-github/rust-leetcode/";

type MyResult<T> = Result<T, Box<dyn Error>>;

/*
  describes the names and types of the arguments
*/
#[derive(Debug)]
pub struct Config {
  question_ids: Vec<String>,
  reload: bool,
  output_folder: String
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
  
  for id in config.question_ids {
    
  }
  Ok(())
}

/*

*/
fn reload_questions_data() {
  let resp_text = reqwest::get(LEETCODE_PROBLEMS_API_URL)
  .expect("")
  .text().expect("");

  println!("{}", resp_text);


  // match  reqwest::get(LEETCODE_PROBLEMS_API_URL){
  //   Ok(mut response) =>{
  //     if response.status() == reqwest::StatusCode::OK {
  //       match response.text(){
  //         Ok(text) => println!("Response Text:{}", text),
  //         Err(_) => println!("Could not read response.")
  //       }
  //     } else {
  //       println!("Response was not 200 OK.");
  //     }
  //   }
  //   //Err(_) => println!("Coule not make the request")
  // }
}