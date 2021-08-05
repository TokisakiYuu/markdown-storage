use juniper::{RootNode, EmptySubscription};
use super::objects::{QueryRoot, MutationRoot, Ctx};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Ctx>>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn save_to_file() {
  let schema = create_schema();
  std::fs::write("schema.graphql", schema.as_schema_language()).unwrap();
}

#[cfg(test)]
mod tests {
  use super::*;
  use juniper::{execute_sync, Variables};
  use crate::serde_json;
  use crate::graphql::objects::Ctx;

  #[test]
  fn query_tag() {
    let schema: Schema = create_schema();
    let (res, _errors) = execute_sync("query { tag { id, name } }", None, &schema, &Variables::new(), &Ctx::default()).unwrap();
    println!("response data: \n{}", serde_json::to_string_pretty(&res).unwrap());
  }

  #[test]
  fn create_article() {
    let schema: Schema = create_schema();
    let document = 
    "mutation {
      createArticle(input: {
        shorthand: \"are you eatting brocoli today?\",
        title: \"brocli is good\",
        content: \"brocli is good!!!\",
        contentType: markdown,
        tags: [\"food\"],
        topic: \"nullable\"
      }) { 
        id,
        shorthand,
        title,
        content,
        contentType,
        hidden,
        lastModify: updateAt,
        createAt
      }
    }".trim();
    let (res, _errors) = execute_sync(document, None, &schema, &Variables::new(), &Ctx::default()).unwrap();
    println!("response data: \n{}", serde_json::to_string_pretty(&res).unwrap());
  }
}