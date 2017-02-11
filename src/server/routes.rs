use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Handler as HyperHandler;
use hyper::server::Listening as HyperListening;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;
use hyper::server::Server as HyperServer;
use hyper::status::StatusCode as HyperStatusCode;
use hyper::uri::RequestUri as HyperRequestUri;

use regex::Captures as RegexCaptures;
use regex::Regex;

use serde_json;

use server::*;
use settings::*;
use upstream::*;

pub type RouteHandlerFn =
	Fn (
		Arc <Mutex <ServerState>>,
		Arc <Upstream>,
		RegexCaptures,
		HyperRequest,
		HyperResponse,
	) + Sync;

pub type RouteHandler =
	& 'static RouteHandlerFn;

macro_rules! route {

	($regex: ident, $handler: ident) => {
		(
			Regex::new ($regex).unwrap (),
			$handler,
		)
	}

}

lazy_static! {

	pub static ref ROUTES: Vec <(Regex, RouteHandler)> = vec! [

		route! (
			ROUTE_RAW_RESOURCE_REGEX,
			ROUTE_RAW_RESOURCE_HANDLER),

		route! (
			ROUTE_RAW_RESOURCES_REGEX,
			ROUTE_RAW_RESOURCES_HANDLER),

	];

}

// ex: noet ts=4 filetype=rust
