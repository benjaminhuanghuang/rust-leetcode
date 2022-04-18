#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub question_id: i64,
    pub frontend_question_id: i64,
    pub question__title: i64,
    pub question__title_slug: String,
    pub is_new_question: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Difficulty {
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct stat_status_pair{
  pub stat: Stat,
  pub difficulty: Difficulty,
  pub pad_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct leetcode_proble_detail{
  pub id: String, 
  pub frontend_id: String,
  pub title: String, 
  pub title_slug: String, 
  pub content: 

}
