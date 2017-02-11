use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;

use regex::Captures as RegexCaptures;

use serde_json;

use routes::*;
use server::*;
use upstream::*;

pub const ROUTE_RAW_KEY_REGEX: & 'static str =
	"^/raw/key(/.+)$";

pub const ROUTE_RAW_KEY_HANDLER: & 'static RouteHandlerFn =
	& route_raw_key;

fn route_raw_key (
	state: Arc <Mutex <ServerState>>,
	upstream: Arc <Upstream>,
	captures: RegexCaptures,
	request: HyperRequest,
	mut response: HyperResponse,
) {

	let key =
		captures.get (1).unwrap ().as_str ();

	let node: Option <Arc <NodeData>> = {

		let data =
			upstream.data ();

		let data =
			data.lock ().unwrap ();

		data.get (
			key,
		).map (
			|value| value.clone (),
		)

	};

	if node.is_none () {

		return send_not_found (
			response);

	}

	let node = node.unwrap ();

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
		"{{\n  \"value\":",
	).unwrap ();

	serde_json::to_writer (
		& mut response,
		& node.value (),
	).unwrap ();

	write! (
		response,
		"\n}}\n",
	).unwrap ();

}

// ex: noet ts=4 filetype=rust
