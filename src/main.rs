mod lib;
use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_sync_method("say_hello", |_| {
        lib::parse_markdown();
		Ok(Value::String("hello".into()))
	});

    io.add_sync_method("save", |params| {
        if let Params::Map(map) = params {
            println!("{:?}", map);
            Ok(Value::Bool(true))
        } else {
            Ok(Value::Bool(false))
        }
    });


	let server = ServerBuilder::new(io)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
