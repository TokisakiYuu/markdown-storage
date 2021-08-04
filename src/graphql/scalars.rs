// use chrono::{DateTime, Utc};
// use juniper::{
//   graphql_scalar, ScalarValue, Value, InputValue, ParseScalarResult, ParseScalarValue
// };

// type Date = DateTime<Utc>;

// #[graphql_scalar(description = "时间类型")]
// impl<S> GraphQLScalar for Date
// where
//   S: ScalarValue
// {
//   fn resolve(&self) -> Value {
//     Value::scalar(self.format("%Y-%m-%d %H:%M:%S").to_string())
//   }
//   fn from_input_value(inp: &InputValue) -> Option<Date> {
//     inp.as_scalar_value()
//       .and_then(|v| v.as_str())
//       .and_then(|s| {
//         match s.parse::<Date>() {
//           Ok(data) => Some(data),
//           _ => None
//         }
//       })
//   }
//   fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
//     match (value.to_string()).parse::<Date>() {
//       Ok(date) => Ok(Value::object(date)),
//       Err(_) => Err(juniper::ParseError::LexerError)
//     }
//   }
// }