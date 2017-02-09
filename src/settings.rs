use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use serde_yaml;

#[ derive (Serialize, Deserialize) ]
pub struct Settings {

	#[ serde (rename = "upstream") ]
	pub upstream: UpstreamSettings,

	#[ serde (rename = "server") ]
	pub server: ServerSettings,

}

#[ derive (Serialize, Deserialize) ]
pub struct UpstreamSettings {

	#[ serde (rename = "ca-certificate") ]
	pub ca_certificate: PathBuf,

	#[ serde (rename = "certificate") ]
	pub certificate: PathBuf,

	#[ serde (rename = "private-key") ]
	pub private_key: PathBuf,

	#[ serde (rename = "server-names") ]
	pub server_names: Vec <String>,

	#[ serde (rename = "server-port") ]
	pub server_port: u16,

	#[ serde (rename = "key-prefix") ]
	pub key_prefix: String,

}

#[ derive (Serialize, Deserialize) ]
pub struct ServerSettings {

	#[ serde (rename = "listen-address") ]
	pub listen_address: String,

	#[ serde (rename = "listen-port") ]
	pub listen_port: u16,

}

impl Settings {

	pub fn load <
		SettingsPath: AsRef <Path>,
	> (
		settings_path: SettingsPath,
	) -> Result <Settings, String> {

		Self::load_impl (
			settings_path.as_ref (),
		)

	}

	fn load_impl (
		settings_path: & Path,
	) -> Result <Settings, String> {

		let settings_file =
			File::open (
				settings_path,
			).map_err (|error|
				format! (
					"Error opening {}: {}",
					settings_path.to_string_lossy (),
					error.description ())
			) ?;

		serde_yaml::from_reader (
			settings_file,
		).map_err (|error|
			format! (
				"Error reading {}: {}",
				settings_path.to_string_lossy (),
				error.description ())
		)

	}

}

// ex: noet ts=4 filetype=rust
