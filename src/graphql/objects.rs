use juniper::{graphql_object, FieldResult, ID};
use std::collections::HashMap;
use super::entities::*;
use super::enums::*;

#[derive(Debug)]
pub struct Ctx {
  users: HashMap<i32, String>
}

impl juniper::Context for Ctx {}

impl Default for Ctx {
  fn default() -> Self {
    Ctx {
      users: HashMap::<i32, String>::new()
    }
  }
}

pub struct QueryRoot;

#[graphql_object(context = Ctx)]
impl QueryRoot {
  fn tag(_context: &Ctx) -> FieldResult<Tag> {
    Ok(Tag {
      id: ID::from("2333".to_owned()),
      name: "hucai".to_owned(),
      comment: "hello world".to_owned(),
      create_at: "2021-5-1".to_owned()
    })
  }

  fn article() -> FieldResult<Article> {
    Ok(Article {
      id: ID::from("123".to_owned()),
      shorthand: "dhuwhiuf".to_owned(),
      title: "eat huacai".to_owned(),
      content: "duhaiu idjowijooops".to_owned(),
      content_type: ArticleContentType::MARKDOWN,
      tags: None,
      topic: None,
      hidden: false,
      update_at: "2021-07-01".to_owned(),
      create_at: "2021-07-01".to_owned()
    })
  }

  fn topic() -> FieldResult<Topic> {
    Ok(Topic {
      id: ID::from("123".to_owned()),
      name: "facai".to_owned(),
      description: "duhaiuwfiuw".to_owned(),
      create_at: "2021-09-01".to_owned()
    })
  }
}

pub struct MutationRoot;

#[graphql_object(context = Ctx)]
impl MutationRoot {
  #[graphql(name = "createArticle")]
  fn create_article(_context: &Ctx, input: ArticleInput) -> FieldResult<Article> {
    Ok(Article {
      id: ID::from("123".to_owned()),
      shorthand: input.shorthand,
      title: input.title,
      content: input.content,
      content_type: input.content_type,
      tags: input.tags,
      topic: input.topic,
      hidden: false,
      update_at: "2021-07-01".to_owned(),
      create_at: "2021-07-01".to_owned()
    })
  }
}