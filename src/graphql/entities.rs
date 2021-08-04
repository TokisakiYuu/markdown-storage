use juniper::{
  GraphQLObject, ID, GraphQLInputObject
};
use super::enums::ArticleContentType;

#[derive(GraphQLObject)]
#[graphql(description = "文章")]
pub struct Article {
  id: ID,
  shorthand: String,
  title: String,
  content: String,
  #[graphql(name = "contentType")]
  content_type: ArticleContentType,
  tags: Option<Vec<String>>,
  topic: Option<String>,
  hidden: bool,
  #[graphql(name = "updateAt")]
  update_at: String,
  #[graphql(name = "createAt")]
  create_at: String
}

#[derive(GraphQLObject)]
#[graphql(description = "标签")]
pub struct Tag {
  pub id: ID,
  pub name: String,
  pub comment: String,
  #[graphql(name = "createAt")]
  pub create_at: String
}

#[derive(GraphQLObject)]
#[graphql(description = "主题")]
pub struct Topic {
  id: ID,
  name: String,
  description: String,
  #[graphql(name = "createAt")]
  create_at: String
}

#[derive(GraphQLInputObject)]
pub struct ArticleInput {
  shorthand: String,
  title: String,
  content: String,
  #[graphql(name = "contentType", default = "ArticleContentType::MARKDOWN")]
  content_type: ArticleContentType,
  tags: Option<Vec<String>>,
  topic: Option<String>
}