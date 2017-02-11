use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Handler as HyperHandler;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::status::StatusCode as HyperStatusCode;
use hyper::uri::RequestUri as HyperRequestUri;

use routes::*;
use server::*;
use upstream::*;

pub struct ServerHandler {
	pub state: Arc <Mutex <ServerState>>,
	pub upstream: Arc <Upstream>,
}

impl HyperHandler for ServerHandler {

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

			send_not_found (
				response,
			);

		} else {

			send_error (
				response,
			);

		}

	}

}

pub fn send_not_found (
	mut response: HyperResponse,
) {

	* response.status_mut () =
		HyperStatusCode::NotFound;

	{

		let headers =
			response.headers_mut ();

		headers.set (
			header::ContentType::plaintext ());

	}

	response.send (
		b"NOT FOUND\n",
	).unwrap ();

}

pub fn send_error (
	mut response: HyperResponse,
) {

	* response.status_mut () =
		HyperStatusCode::InternalServerError;

	{

		let headers =
			response.headers_mut ();

		headers.set (
			header::ContentType::plaintext ());

	}

	response.send (
		b"ERROR\n",
	).unwrap ();

}

// ex: noet ts=4 filetype=rust
