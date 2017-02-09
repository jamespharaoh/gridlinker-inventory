use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

use hyper::client::Client as HyperClient;
use hyper::net::HttpsConnector as HyperHttpsConnector;

use openssl::ssl::SslConnectorBuilder;
use openssl::ssl::SslMethod;
use openssl::x509::X509_FILETYPE_PEM;

use serde_json;

use etcd::*;
use settings::*;
use ssl::*;

pub struct Upstream {
	state: Arc <Mutex <State>>,
	data_thread: JoinHandle <()>,
	hyper_client: Arc <HyperClient>,
}

struct State {
	data: HashMap <String, NodeData>,
	etcd_index: u64,
}

struct NodeData {
	value: String,
	created_index: u64,
	modified_index: u64,
}

impl Upstream {

	pub fn start (
		settings: Arc <Settings>,
	) -> Result <Upstream, String> {

		let hyper_client =
			Arc::new (
				Self::hyper_client_create (
					settings.clone (),
				) ?
			);

		let (resource_data, etcd_index) =
			Self::resource_data_load_initial (
				settings.clone (),
				hyper_client.clone (),
			) ?;

		let state =
			Arc::new (Mutex::new (
				State {
					data: resource_data,
					etcd_index: etcd_index,
				}
			));

		let data_thread = {

			let state = state.clone ();
			let hyper_client = hyper_client.clone ();

			thread::spawn (move ||
				Self::data_thread (
					settings,
					state,
					hyper_client)
			)

		};

		Ok (Upstream {
			state: state,
			data_thread: data_thread,
			hyper_client: hyper_client,
		})

	}

	fn hyper_client_create (
		settings: Arc <Settings>,
	) -> Result <HyperClient, String> {

		let mut ssl_connector_builder =
			SslConnectorBuilder::new (
				SslMethod::tls (),
			).map_err (|error|
				format! (
					"Error initialising openssl: {}",
					error.description ())
			) ?;

		{

			let mut ssl_context_builder =
				ssl_connector_builder.builder_mut ();

			ssl_context_builder.set_ca_file (
				& settings.upstream.ca_certificate,
			).map_err (|error|
				format! (
					"Error loading ca certificate {}: {}",
					settings.upstream.ca_certificate.to_string_lossy (),
					error.description ())
			) ?;

			ssl_context_builder.set_certificate_file (
				& settings.upstream.certificate,
				X509_FILETYPE_PEM,
			).map_err (|error|
				format! (
					"Error loading certificate {}: {}",
					settings.upstream.certificate.to_string_lossy (),
					error.description ())
			) ?;

			ssl_context_builder.set_private_key_file (
				& settings.upstream.private_key,
				X509_FILETYPE_PEM,
			).map_err (|error|
				format! (
					"Error loading private key {}: {}",
					settings.upstream.private_key.to_string_lossy (),
					error.description ())
			) ?;

		}

		let hyper_connector =
			HyperHttpsConnector::new (
				OpensslClient::new (
					ssl_connector_builder.build ()));

		let hyper_client =
			HyperClient::with_connector (
				hyper_connector);

		Ok (hyper_client)

	}

	fn resource_data_load_initial (
		settings: Arc <Settings>,
		hyper_client: Arc <HyperClient>,
	) -> Result <(HashMap <String, NodeData>, u64), String> {

		println! (
			"Load initial data ...");

		let server_url =
			format! (
				"https://{}:{}/v2/keys{}/resource?recursive=true",
				settings.upstream.server_names [0],
				settings.upstream.server_port,
				settings.upstream.key_prefix);

		let mut response =
			hyper_client.get (
				& server_url,
			).send (
			).map_err (|error|
				format! (
					"Error connecting to {}: {}",
					server_url,
					error.description ())
			) ?;

		let etcd_index =
			str::from_utf8 (
				& response.headers.get_raw (
					"X-Etcd-Index",
				).unwrap () [0]
			).unwrap ().parse ().unwrap ();

		let mut response_string =
			String::new ();

		response.read_to_string (
			& mut response_string,
		).map_err (|error|
			format! (
				"Error reading from {}: {}",
				server_url,
				error.description ())
		) ?;

		let response_data: EtcdResponse =
			serde_json::from_str (
				& mut response_string,
			).map_err (|error|
				format! (
					"Error decoding response from {}: {}",
					server_url,
					error.description ())
			) ?;

		let mut resource_data: HashMap <String, NodeData> =
			HashMap::new ();

		Self::store_node_recursive (
			& mut resource_data,
			& response_data.node);

		println! (
			"Got {} nodes, etcd index is {}",
			resource_data.len (),
			etcd_index);

		Ok (
			(
				resource_data,
				etcd_index,
			)
		)

	}

	fn store_node_recursive (
		data: & mut HashMap <String, NodeData>,
		node: & EtcdNode,
	) {

		if node.dir {

			for child_node in node.nodes.iter () {

				Self::store_node_recursive (
					data,
					child_node,
				);

			};

		} else {

			match node.value {

				Some (ref value) =>
					data.insert (
						node.key.to_owned (),
						NodeData {
							value: value.to_owned (),
							created_index: node.created_index,
							modified_index: node.modified_index,
						},
					),

				None =>
					data.remove (
						& node.key),

			};

		}

	}

	fn data_thread (
		settings: Arc <Settings>,
		state: Arc <Mutex <State>>,
		hyper_client: Arc <HyperClient>,
	) {

		loop {

			Self::data_thread_watch (
				settings.clone (),
				state.clone (),
				hyper_client.clone (),
			).unwrap_or_else (|error|
				panic! (
					"Error in background thread: {}",
					error)
			);

		}

	}

	fn data_thread_watch (
		settings: Arc <Settings>,
		state: Arc <Mutex <State>>,
		hyper_client: Arc <HyperClient>,
	) -> Result <(), String> {

		let etcd_index =
			state.lock ().unwrap ().etcd_index;

		let server_url =
			format! (
				"https://{}:{}/v2/keys{}/resource\
				?wait=true\
				&waitIndex={}\
				&recursive=true",
				settings.upstream.server_names [0],
				settings.upstream.server_port,
				settings.upstream.key_prefix,
				etcd_index + 1);

		let mut response =
			hyper_client.get (
				& server_url,
			).send (
			).map_err (|error|
				format! (
					"Error connecting to {}: {}",
					server_url,
					error.description ())
			) ?;

		let mut response_string =
			String::new ();

		response.read_to_string (
			& mut response_string,
		).map_err (|error|
			format! (
				"Error reading from {}: {}",
				server_url,
				error.description ())
		) ?;

		let response_data: EtcdResponse =
			serde_json::from_str (
				& mut response_string,
			).map_err (|error|
				format! (
					"Error decoding response from {}: {}",
					server_url,
					error.description ())
			) ?;

		{

			let mut state =
				state.lock ().unwrap ();

			state.etcd_index =
				str::from_utf8 (
					& response.headers.get_raw (
						"X-Etcd-Index",
					).unwrap () [0]
				).unwrap ().parse ().unwrap ();

			Self::store_node_recursive (
				& mut state.data,
				& response_data.node);

			println! (
				"Update: Got {} nodes, etcd index is {}",
				state.data.len (),
				state.etcd_index);

		}

		Ok (())

	}

}

// ex: noet ts=4 filetype=rust
