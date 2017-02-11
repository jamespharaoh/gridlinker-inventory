use std::io::Write;
use std::sync::Arc;
use std::sync::Mutex;

use hyper::header;
use hyper::server::Request as HyperRequest;
use hyper::server::Response as HyperResponse;

use regex::Captures as RegexCaptures;

use serde_json;

use server::*;
use upstream::*;

pub const ROUTE_RAW_RESOURCES_REGEX: & 'static str =
	"^/raw/resources$";

pub const ROUTE_RAW_RESOURCES_HANDLER: & 'static RouteHandlerFn =
	& route_raw_resources;

fn route_raw_resources (
	state: Arc <Mutex <ServerState>>,
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
