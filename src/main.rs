// mod lib;
mod db;
use db::MDDB;
use std::fs;
// use jsonrpc_http_server::*;
// use jsonrpc_http_server::jsonrpc_core::*;
// use jsonrpc_http_server::jsonrpc_core::serde_json::json;

fn main() {
    let content = fs::read_to_string("./assets/test.md").unwrap();

    let db = MDDB::open();
    db.save_markdown(content);

	// let mut io = IoHandler::default();
	// io.add_sync_method("say_hello", |_| {
    //     lib::parse_markdown();
	// 	Ok(Value::String("hello".into()))
	// });

    // io.add_sync_method("save", |params| {
    //     if let Params::Map(map) = params {
    //         println!("{:?}", map);
    //         Ok(json!({
    //             "ok": true
    //         }))
    //     } else {
    //         Ok(json!({
    //             "ok": false
    //         }))
    //     }
    // });


	// let server = ServerBuilder::new(io)
	// 	.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
	// 	.start_http(&"127.0.0.1:3030".parse().unwrap())
	// 	.expect("Unable to start RPC server");

	// server.wait();
}
