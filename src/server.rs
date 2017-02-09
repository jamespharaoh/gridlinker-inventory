use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Handler as HyperHandler;
use hyper::server::Listening as HyperListening;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::server::Server as HyperServer;

use settings::*;

pub struct Server {
	hyper_listening: HyperListening,
	state: Arc <Mutex <State>>,
}

struct State {
	counter: u64,
}

struct Handler {
	state: Arc <Mutex <State>>,
}

impl Server {

	pub fn start (
		settings: Arc <Settings>,
	) -> Result <Server, String> {

		let state =
			Arc::new (Mutex::new (
				State {
					counter: 0,
				}
			));

		let handler =
			Handler {
				state: state.clone (),
			};

		let hyper_server =
			HyperServer::http (
				(
					settings.server.listen_address.as_str (),
					settings.server.listen_port,
				),
			).map_err (|error|
				format! (
					"Error creating server: {}",
					error.description ())
			) ?;

		let hyper_listening =
			hyper_server.handle (
				handler,
			).map_err (|error|
				format! (
					"Error starting server: {}",
					error.description ())
			) ?;

		Ok (Server {
			hyper_listening: hyper_listening,
			state: state,
		})

	}

}

impl HyperHandler for Handler {

	fn handle (
		& self,
		request: HyperRequest,
		mut response: HyperResponse,
	) {

		{

			let headers =
				response.headers_mut ();

			headers.set (
				header::ContentType::plaintext ());

		}

		let counter = {

			let mut state =
				self.state.lock ().unwrap ();

			let counter =
				state.counter;

			state.counter += 1;

			counter

		};

		response.send (
			format! (
				"{}\n",
				counter,
			).as_bytes (),
		).unwrap ();

	}

}

// ex: noet ts=4 filetype=rust
