use juniper::{
  GraphQLObject, ID, GraphQLInputObject
};
use super::enums::ArticleContentType;

#[derive(GraphQLObject)]
#[graphql(description = "文章")]
pub struct Article {
  pub id: ID,
  pub shorthand: String,
  pub title: String,
  pub content: String,
  #[graphql(name = "contentType")]
  pub content_type: ArticleContentType,
  pub tags: Option<Vec<String>>,
  pub topic: Option<String>,
  pub hidden: bool,
  #[graphql(name = "updateAt")]
  pub update_at: String,
  #[graphql(name = "createAt")]
  pub create_at: String
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
  pub id: ID,
  pub name: String,
  pub description: String,
  #[graphql(name = "createAt")]
  pub create_at: String
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "新建文章的表单对象")]
pub struct ArticleInput {
  pub shorthand: String,
  pub title: String,
  pub content: String,
  #[graphql(name = "contentType", default = "ArticleContentType::MARKDOWN")]
  pub content_type: ArticleContentType,
  pub tags: Option<Vec<String>>,
  pub topic: Option<String>
}