use juniper::{graphql_object, FieldResult, ID};
use super::entities::{Tag};

pub struct QueryRoot;

#[graphql_object]
impl QueryRoot {
  fn tag() -> FieldResult<Tag> {
    Ok(Tag {
      id: ID::from("2333".to_owned()),
      name: "hucai".to_owned(),
      comment: "hello world".to_owned(),
      create_at: "2021-5-1".to_owned()
    })
  }
}

pub struct MutationRoot;

#[graphql_object]
impl MutationRoot {
  fn create_tag(name: String) -> FieldResult<Tag> {
    Ok(Tag {
      id: ID::from("2333".to_owned()),
      name: name.to_owned(),
      comment: name.to_owned(),
      create_at: "2021-5-1".to_owned()
    })
  }
}