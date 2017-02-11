use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::server::Listening as HyperListening;
use hyper::server::Server as HyperServer;

use server::*;
use settings::*;
use upstream::*;

pub struct Server {
	hyper_listening: HyperListening,
	state: Arc <Mutex <ServerState>>,
}

impl Server {

	pub fn start (
		settings: Arc <Settings>,
		upstream: Arc <Upstream>,
	) -> Result <Server, String> {

		let state =
			Arc::new (Mutex::new (
				ServerState {
					counter: 0,
				}
			));

		let handler =
			ServerHandler {
				state: state.clone (),
				upstream: upstream,
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

		println! (
			"Listening on {}:{}",
			settings.server.listen_address,
			settings.server.listen_port);

		Ok (Server {
			hyper_listening: hyper_listening,
			state: state,
		})

	}

}

// ex: noet ts=4 filetype=rust
