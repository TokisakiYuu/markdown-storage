use juniper::{
  GraphQLEnum
};

#[derive(GraphQLEnum)]
#[graphql(description = "文章内容的类型")]
pub enum ArticleContentType {
  #[graphql(name = "html")]
  HTML,
  #[graphql(name = "link")]
  LINK,
  #[graphql(name = "plain")]
  PLAIN,
  #[graphql(name = "markdown")]
  MARKDOWN
}