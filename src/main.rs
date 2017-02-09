#[ macro_use ]
extern crate lazy_static;

#[ macro_use ]
extern crate serde_derive;

extern crate hyper;
extern crate libc;
extern crate openssl;
extern crate regex;
extern crate serde_json;
extern crate serde_yaml;

mod daemon;
mod etcd;
mod settings;
mod server;
mod ssl;
mod upstream;

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use daemon::*;
use settings::*;

fn main () {

	match main_impl () {

		Ok (exit_code) =>
			process::exit (exit_code),

		Err (error) => {

			println! (
				"{}",
				error);

			process::exit (1);

		}

	}

}

fn main_impl (
) -> Result <i32, String> {

	let arguments: Vec <OsString> =
		env::args_os ().collect ();

	if arguments.len () != 2 {

		return Err (
			format! (
				"Syntax: {} <SETTINGS-PATH>",
				arguments [0].to_string_lossy ())
		);

	}

	let settings_path =
		PathBuf::from (
			arguments [1].to_owned (),
		);

	println! (
		"Using settings: {}",
		settings_path.to_string_lossy ());

	let settings =
		Arc::new (
			Settings::load (
				settings_path,
			) ?
		);

	Ok (daemon_start (
		settings.clone (),
	))

}

// ex: noet ts=4 filetype=rust
