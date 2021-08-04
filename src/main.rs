// mod lib;
mod db;
mod types;
mod graphql;
use db::MDDB;
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::jsonrpc_core::serde_json::json;
use std::sync::{Arc, Mutex};

// fn main() {
//     let db = Arc::new(Mutex::new(MDDB::open()));
//     let db_clone = db.clone();

// 	let mut io = IoHandler::default();
// 	io.add_sync_method("get", move |params| {
//         if let Params::Map(map) = params {
//             match map.get("hash") {
//                 Some(hash) => {
//                     let db = (*db).lock().unwrap();
//                     match db.get_markdown(hash.as_str().unwrap_or("")) {
//                         Some((meta_data, source)) => {
//                             Ok(json!({
//                                 "ok": true,
//                                 "meta_data": &meta_data,
//                                 "content": source
//                             }))
//                         },
//                         None => {
//                             Ok(json!({
//                                 "ok": false,
//                                 "reason": "not found markdown"
//                             }))
//                         }
//                     }
//                 },
//                 None => {
//                     Ok(json!({
//                         "ok": false,
//                         "reason": "missing paramenter"
//                     }))
//                 }
//             }
//         } else {
//             Ok(json!({
//                 "ok": false,
//                 "reason": "paramenter error"
//             }))
//         }
// 	});

//     io.add_sync_method("save", move |params| {
//         if let Params::Map(map) = params {
//             match map.get("content") {
//                 Some(content) => {
//                     if let Some(content) = content.as_str() {
//                         let db = (*db_clone).lock().unwrap();
//                         let hash = db.save_markdown(content.to_owned());
//                         Ok(json!({
//                             "ok": true,
//                             "hash": hash
//                         }))
//                     } else {
//                         Ok(json!({
//                             "ok": false,
//                             "reason": "paramenter type error"
//                         }))
//                     }
//                 },
//                 None => {
//                     Ok(json!({
//                         "ok": false,
//                         "reason": "missing paramenter"
//                     }))
//                 }
//             }
//         } else {
//             Ok(json!({
//                 "ok": false
//             }))
//         }
//     });


// 	let server = ServerBuilder::new(io)
// 		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
// 		.start_http(&"127.0.0.1:3030".parse().unwrap())
// 		.expect("Unable to start RPC server");

// 	server.wait();
// }

fn main() {
    graphql::schema::save_to_file();
}