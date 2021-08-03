use jammdb::{DB};
use rmp_serde;
use markdown_parser::{parse_format, Format};
use yaml_rust::{YamlLoader, Yaml};
use md5;
// use chrono::{Utc, DateTime};
use crate::types::{MarkdownMetadata};
use std::str;

pub struct MDDB {
  db: DB
}

impl MDDB {
  pub fn open() -> Self {
    if let Ok(db) = DB::open("store.db") {
      MDDB {
        db: db
      }
    } else { panic!() }
  }

  pub fn save_markdown(&self, content: String) -> String {
    let (meta_data, source) = MDDB::parse_markdown(content);
    let hash = meta_data.source_hash.clone();
    self.save_metadata(&hash, &meta_data);
    self.save_source(&hash, source);
    hash
  }

  fn save_metadata(&self, hash: &String, data: &MarkdownMetadata) {
    let trans = self.db.tx(true).unwrap();
    let meta_data_bucket = trans.get_or_create_bucket("meta_data").unwrap();
    let user_bytes = rmp_serde::to_vec(&data).unwrap();
    meta_data_bucket.put(hash, user_bytes).unwrap();
    trans.commit().unwrap();
  }

  fn save_source(&self, hash: &String, content: String) {
    let trans = self.db.tx(true).unwrap();
    let contents_bucket = trans.get_or_create_bucket("contents").unwrap();
    contents_bucket.put(hash, content).unwrap();
    trans.commit().unwrap();
  }

  fn parse_markdown(content: String) -> (MarkdownMetadata, String) {
    let md_parties = parse_format(&content, Format::YAML).unwrap();
    let metadata = YamlLoader::load_from_str(md_parties.front_matter()).unwrap();
    let yaml = &metadata[0];
    let shortcut = yaml["shortcut"].as_str().unwrap_or("");
    let tags_vec = yaml["tags"].as_vec().unwrap();
    let mut tags = vec!();
    for t in tags_vec {
      if let Yaml::String(tag) = t {
        tags.push(String::from(tag));
      }
    }
    let topic = yaml["topic"].as_str().unwrap_or("");
    let create_at = yaml["createAt"].as_str().unwrap_or("");
    let update_at = yaml["updateAt"].as_str().unwrap_or("");
    let source_hash = format!("{:x}", md5::compute(content));
    let meta_data = MarkdownMetadata{
      source_hash,
      shortcut: shortcut.to_string(),
      tags: tags,
      topic: topic.to_string(),
      create_at: create_at.to_string(),
      update_at: update_at.to_string()
    };
    (meta_data, md_parties.content().clone())
  }

  pub fn get_markdown(&self, hash: &str) -> Option<(MarkdownMetadata, String)> {
    let hash_string = &hash.to_owned();
    let meta_data = self.get_metadata(hash_string)?;
    let source = self.get_source(hash_string)?;
    Some((meta_data, source))
  }

  fn get_metadata(&self, hash: &String) -> Option<MarkdownMetadata> {
    let trans = self.db.tx(false).unwrap();
    match trans.get_bucket("meta_data") {
      Err(_) => None,
      Ok(meta_data_bucket) => {
        if let Some(data) = meta_data_bucket.get(hash) {
          if data.is_kv() {
            let kv = data.kv();
            Some(rmp_serde::from_slice(kv.value()).unwrap())
          } else { None }
        } else { None }
      }
    }
  }

  fn get_source(&self, hash: &String) -> Option<String> {
    let trans = self.db.tx(false).unwrap();
    match trans.get_bucket("contents") {
      Err(_) => None,
      Ok(contents_bucket) => {
        if let Some(data) = contents_bucket.get(hash) {
          if data.is_kv() {
            match str::from_utf8(data.kv().value()) {
              Ok(content) => Some(content.to_owned()),
              Err(_) => None
            }
          } else { None }
        } else { None }
      }
    }
  }
}