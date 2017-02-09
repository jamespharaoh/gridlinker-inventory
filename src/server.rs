use std::error::Error;
use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Handler as HyperHandler;
use hyper::server::Listening as HyperListening;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::server::Server as HyperServer;
use hyper::uri::RequestUri as HyperRequestUri;

use regex::Captures as RegexCaptures;
use regex::Regex;

use serde_json;

use settings::*;
use upstream::*;

pub struct Server {
	hyper_listening: HyperListening,
	state: Arc <Mutex <State>>,
}

struct State {
	counter: u64,
}

struct Handler {
	state: Arc <Mutex <State>>,
	upstream: Arc <Upstream>,
}

impl Server {

	pub fn start (
		settings: Arc <Settings>,
		upstream: Arc <Upstream>,
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

impl HyperHandler for Handler {

	fn handle (
		& self,
		request: HyperRequest,
		mut response: HyperResponse,
	) {

		let uri =
			request.uri.clone ();

		if let HyperRequestUri::AbsolutePath (path) =
			uri {

			for & (ref route_regex, ref route_handler)
			in ROUTES.iter () {

				if let Some (route_captures) =
					route_regex.captures (
						& path) {

					return route_handler (
						self.state.clone (),
						self.upstream.clone (),
						route_captures,
						request,
						response,
					);

				}

			}

			response.send (
				b"NOT FOUND\n",
			).unwrap ();

		} else {

			response.send (
				b"ERROR\n",
			).unwrap ();

		}

	}

}

type RouteHandlerFn =
	Fn (
		Arc <Mutex <State>>,
		Arc <Upstream>,
		RegexCaptures,
		HyperRequest,
		HyperResponse,
	) + Sync;

type RouteHandler =
	& 'static RouteHandlerFn;

lazy_static! {

	static ref ROUTES: Vec <(Regex, RouteHandler)> = vec! [
		(
			Regex::new (
				"^/raw/resources$"
			).unwrap (),
			ROUTE_RESOURCES,
		),
	];

}

const ROUTE_RESOURCES: & 'static RouteHandlerFn =
	& route_resources;

fn route_resources (
	state: Arc <Mutex <State>>,
	upstream: Arc <Upstream>,
	captures: RegexCaptures,
	request: HyperRequest,
	mut response: HyperResponse,
) {

	let resources_temp: Vec <Arc <NodeData>> = {

		let data =
			upstream.data ();

		let data =
			data.lock ().unwrap ();

		data.iter ().filter (
			|& (ref key, ref _node)|
			key.starts_with ("/resource/")
			&& key.ends_with ("/data")
		).map (
			|(ref _key, ref node)|
			(* node).clone ()
		).collect ()

	};

	{

		let mut headers =
			response.headers_mut ();

		headers.set (
			header::ContentType::json ());

	}

	let mut response =
		response.start ().unwrap ();

	let mut first = true;

	write! (
		response,
		"{{\n  \"resources\": [",
	).unwrap ();

	for resource in resources_temp {

		if first {

			write! (
				response,
				"\n    ",
			).unwrap ();

			first = false;

		} else {

			write! (
				response,
				",\n    ",
			).unwrap ();

		}

		serde_json::to_writer (
			& mut response,
			& resource.value (),
		).unwrap ();

	}

	write! (
		response,
		"\n  ]\n}}\n",
	).unwrap ();

}

// ex: noet ts=4 filetype=rust
