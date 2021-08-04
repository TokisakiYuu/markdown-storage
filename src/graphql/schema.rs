use juniper::{RootNode, EmptySubscription};
use super::objects::{QueryRoot, MutationRoot};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub fn save_to_file() {
  let schema = create_schema();
  std::fs::write("schema.graphql", schema.as_schema_language()).unwrap();
}