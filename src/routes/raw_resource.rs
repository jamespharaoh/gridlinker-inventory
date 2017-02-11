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

pub const ROUTE_RAW_RESOURCE_REGEX: & 'static str =
	"^/raw/resource/(.+)$";

pub const ROUTE_RAW_RESOURCE_HANDLER: & 'static RouteHandlerFn =
	& route_raw_resource;

fn route_raw_resource (
	state: Arc <Mutex <ServerState>>,
	upstream: Arc <Upstream>,
	captures: RegexCaptures,
	request: HyperRequest,
	mut response: HyperResponse,
) {

	let resource_name =
		captures.get (1).unwrap ().as_str ();

	let resource: Option <Arc <NodeData>> = {

		let data =
			upstream.data ();

		let data =
			data.lock ().unwrap ();

		data.get (
			& format! (
				"/resource/{}/data",
				resource_name),
		).map (
			|value| value.clone (),
		)

	};

	if resource.is_none () {

		return send_not_found (
			response);

	}

	let resource = resource.unwrap ();

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
		"{{\n  \"resource\":",
	).unwrap ();

	serde_json::to_writer (
		& mut response,
		& resource.value (),
	).unwrap ();

	write! (
		response,
		"\n}}\n",
	).unwrap ();

}

// ex: noet ts=4 filetype=rust
