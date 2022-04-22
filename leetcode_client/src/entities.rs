use serde::{Deserialize, Serialize};
use std::fmt;

// region: leetcode json
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Leetcode {
  pub stat_status_pairs: Vec<StatStatusPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatStatusPair {
  pub stat: Stat,
  pub difficulty: Difficulty,
  pub paid_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
  pub question_id: u32,
  pub frontend_question_id: u32,
  pub question__title: String,
  pub question__title_slug: String,
  pub is_new_question: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Difficulty {
  pub level: u32,
}

pub enum Level {
  Easy,
  Medium,
  Hard,
}

impl Level {
  pub fn from_u32(value: u32) -> Level {
    match value {
      1 => Level::Easy,
      2 => Level::Medium,
      3 => Level::Hard,
      _ => panic!("Unknown value: {}", value),
    }
  }
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Level::Easy => write!(f, "Easy"),
        Level::Medium => write!(f, "Medium"),
        Level::Hard => write!(f, "Hard"),
      }
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuesitonData {
  pub question:Question
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
  pub code_snippets:Vec<CodeSnippet>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSnippet {
  pub code: String,
  pub lang: String,
  pub lanSlug: String,
}
// endregion: leetcode json
