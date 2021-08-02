use jammdb::{DB, Error};
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};
use markdown_parser::{parse_format, Format};
use yaml_rust::{YamlLoader, Yaml};
use md5;
use chrono::{Utc, DateTime};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MarkdownMetadata {
  shortcut: String,
  tags: Vec<String>,
  topic: String,
  create_at: String,
  update_at: String,
  source_hash: String
}

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

  pub fn save_markdown(&self, content: String) {
    let md_parties = parse_format(&content, Format::YAML).unwrap();
    let markdown_source = md_parties.content();
    println!("markdown: \n{}\n", markdown_source);
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
    println!("
      shortcut: {},
      tags: {:?},
      topic: {},
      create_at: {},
      update_at: {},
      source_hash: {}
    ", shortcut, tags, topic, create_at, update_at, source_hash);
  }

  // fn get_markdown() {

  // }
}

pub fn jammdb_test() -> Result<(), Error> {
{
    // open a new database file
    let db = DB::open("my-database.db")?;

    // open a writable transaction so we can make changes
    let mut tx = db.tx(true)?;

    // create a bucket to store a map of first names to last names
    let mut names_bucket = tx.create_bucket("names")?;
    names_bucket.put(b"Kanan", b"Jarrus")?;
    names_bucket.put(b"Ezra", b"Bridger")?;

    // commit the changes so they are saved to disk
    tx.commit()?;
}
{
    // open the existing database file
    let db = DB::open("my-database.db")?;
    // open a read-only transaction to get the data
    let mut tx = db.tx(true)?;
    // get the bucket we created in the last transaction
    let names_bucket = tx.get_bucket("names")?;
    // get the key / value pair we inserted into the bucket
    if let Some(data) = names_bucket.get(b"Kanan") {
        assert_eq!(data.kv().value(), b"Jarrus");
    }
}
    Ok(())
}