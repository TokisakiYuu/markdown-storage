use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MarkdownMetadata {
  pub shortcut: String,
  pub tags: Vec<String>,
  pub topic: String,
  pub create_at: String,
  pub update_at: String,
  pub source_hash: String
}