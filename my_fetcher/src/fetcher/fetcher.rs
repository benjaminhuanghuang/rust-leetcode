use super::entities::{CodeDefinition, Problem, Problems, Query, RawProblem};

use clap::{Arg, Command};

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

use regex::Regex;

use serde_json::Value;

#[derive(Debug)]
pub struct Config {
  question_ids: Vec<u32>,
}
type MyResult<T> = Result<T, Box<dyn Error>>;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";

const TARGET_DIR: &str = "../src";

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
  let matches = Command::new("Leetcode Fetcher")
    .version("0.1.0")
    .author("Ben <ben@gmail.com>")
    .about("Rust Leetcode fetcher")
    .arg(
      Arg::new("ids")
        .help("Input problem id(s)")
        .multiple_values(true),
    )
    .get_matches();

  Ok(Config {
    question_ids: matches.values_of_t("ids").unwrap_or_else(|e| e.exit()),
  })
}

pub fn run(config: Config) -> MyResult<()> {
  let problems = get_problems().unwrap();

  for id in config.question_ids.iter() {
    //generate_solution(&config, &id);
    println!("Fetch problem {}", id);
    let problem = get_problem(id, &problems).unwrap_or_else(|| {
      panic!(
        "Error: failed to get problem #{} \
           (The problem may be paid-only or may not be exist).",
        id
      )
    });

    let code = problem
      .code_definition
      .iter()
      .find(|&d| d.value == "rust".to_string());
    if code.is_none() {
      println!("Problem {} has no rust version.", &id);
      continue;
    }
    let code = code.unwrap();
    deal_problem(&problem, &code, true);
  }
  Ok(())
}

/*
  Get problem list

*/
fn get_problems() -> Option<Problems> {
  reqwest::get(PROBLEMS_URL).unwrap().json().unwrap()
}
/*

*/
pub fn get_problem(frontend_question_id: &u32, problems: &Problems) -> Option<Problem> {
  for problem in problems.stat_status_pairs.iter() {
    if problem.stat.frontend_question_id == *frontend_question_id {
      if problem.paid_only {
        return None;
      }

      let client = reqwest::Client::new();
      let resp: RawProblem = client
        .post(GRAPHQL_URL)
        .json(&Query::question_query(
          problem.stat.question_title_slug.as_ref().unwrap(),
        ))
        .send()
        .unwrap()
        .json()
        .unwrap();
      return Some(Problem {
        title: problem.stat.question_title.clone().unwrap(),
        title_slug: problem.stat.question_title_slug.clone().unwrap(),
        code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
        content: resp.data.question.content,
        sample_test_case: resp.data.question.sample_test_case,
        difficulty: problem.difficulty.to_string(),
        question_id: problem.stat.frontend_question_id,
        return_type: {
          let v: Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
          v["return"]["type"].to_string().replace("\"", "")
        },
      });
    }
  }
  None
}

/*
  Generate the files for solution
*/
fn deal_problem(problem: &Problem, code: &CodeDefinition, write_mod_file: bool) {
  let file_name = format!(
    "_{:04}_{}",
    problem.question_id,
    problem.title_slug.replace("-", "_")
  );
  let file_path = Path::new(TARGET_DIR).join(format!("{}.rs", file_name));
  if file_path.exists() {
    panic!("problem already initialized");
  }

  let template = fs::read_to_string("./template.txt").unwrap();
  let source = template
    .replace("__PROBLEM_TITLE__", &problem.title)
    .replace("__PROBLEM_LINK__", &parse_problem_link(problem))
    .replace("__PROBLEM_DIFFICULTY__", &problem.difficulty)
    .replace(
      "__PROBLEM_DEFAULT_CODE__",
      &insert_return_in_code(&problem.return_type, &code.default_code),
    )
    .replace("__PROBLEM_ID__", &format!("{}", problem.question_id))
    .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code));

  let mut file = fs::OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&file_path)
    .unwrap();

  file.write_all(source.as_bytes()).unwrap();
  drop(file);

  if write_mod_file {
    let mut lib_file = fs::OpenOptions::new()
      .write(true)
      .append(true)
      .open(format!("{}/lib.rs", TARGET_DIR))
      .unwrap();
    writeln!(lib_file, "pub mod {};", file_name);
  }
}

fn parse_extra_use(code: &str) -> String {
  let mut extra_use_line = String::new();
  // a linked-list problem
  if code.contains("pub struct ListNode") {
    extra_use_line.push_str("\nuse super::util::list_node::{to_list, ListNode};")
  }
  if code.contains("pub struct TreeNode") {
    extra_use_line.push_str("\nuse super::util::tree_node::{to_tree, TreeNode};")
  }
  // if code.contains("pub struct Point") {
  //   extra_use_line.push_str("\nuse super::util::point::Point;")
  // }
  extra_use_line
}

fn parse_problem_link(problem: &Problem) -> String {
  format!("https://leetcode.com/problems/{}/", problem.title_slug)
}

fn parse_discuss_link(problem: &Problem) -> String {
  format!(
    "https://leetcode.com/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
    problem.title_slug
  )
}

fn insert_return_in_code(return_type: &str, code: &str) -> String {
  let re = Regex::new(r"\{[ \n]+}").unwrap();
  match return_type {
    "ListNode" => re
      .replace(&code, "{\n        Some(Box::new(ListNode::new(0)))\n    }")
      .to_string(),
    "ListNode[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "TreeNode" => re
      .replace(
        &code,
        "{\n        Some(Rc::new(RefCell::new(TreeNode::new(0))))\n    }",
      )
      .to_string(),
    "boolean" => re.replace(&code, "{\n        false\n    }").to_string(),
    "character" => re.replace(&code, "{\n        '0'\n    }").to_string(),
    "character[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "double" => re.replace(&code, "{\n        0f64\n    }").to_string(),
    "double[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "int[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "integer" => re.replace(&code, "{\n        0\n    }").to_string(),
    "integer[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "integer[][]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<String>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<TreeNode>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<boolean>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<double>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<integer>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<list<integer>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<list<string>>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "list<string>" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "null" => code.to_string(),
    "string" => re
      .replace(&code, "{\n        String::new()\n    }")
      .to_string(),
    "string[]" => re.replace(&code, "{\n        vec![]\n    }").to_string(),
    "void" => code.to_string(),
    "NestedInteger" => code.to_string(),
    "Node" => code.to_string(),
    _ => code.to_string(),
  }
}

fn build_desc(content: &str) -> String {
  // TODO: fix this shit
  content
    .replace("<strong>", "")
    .replace("</strong>", "")
    .replace("<em>", "")
    .replace("</em>", "")
    .replace("</p>", "")
    .replace("<p>", "")
    .replace("<b>", "")
    .replace("</b>", "")
    .replace("<pre>", "")
    .replace("</pre>", "")
    .replace("<ul>", "")
    .replace("</ul>", "")
    .replace("<li>", "")
    .replace("</li>", "")
    .replace("<code>", "")
    .replace("</code>", "")
    .replace("<i>", "")
    .replace("</i>", "")
    .replace("<sub>", "")
    .replace("</sub>", "")
    .replace("</sup>", "")
    .replace("<sup>", "^")
    .replace("&nbsp;", " ")
    .replace("&gt;", ">")
    .replace("&lt;", "<")
    .replace("&quot;", "\"")
    .replace("&minus;", "-")
    .replace("&#39;", "'")
    .replace("\n\n", "\n")
    .replace("\n", "\n * ")
}
