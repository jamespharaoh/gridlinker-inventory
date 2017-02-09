#[ macro_use ]
extern crate serde_derive;

extern crate hyper;
extern crate openssl;
extern crate serde_json;
extern crate serde_yaml;

mod etcd;
mod settings;
mod server;
mod ssl;
mod upstream;

use std::process;
use std::sync::Arc;

use settings::*;
use server::*;
use upstream::*;

fn main () {

	if let Err (error) = main_real () {

		println! (
			"{}",
			error);

		process::exit (1);

	}

}

fn main_real (
) -> Result <(), String> {

	let settings =
		Arc::new (
			Settings::load (
				"config/settings",
			) ?
		);

	let upstream =
		Upstream::start (
			settings.clone (),
		) ?;

	println! (
		"Connect to {}:{}",
		settings.upstream.server_names [0],
		settings.upstream.server_port);

	let server =
		Server::start (
			settings.clone (),
		) ?;

	Ok (())

}

// ex: noet ts=4 filetype=rust
