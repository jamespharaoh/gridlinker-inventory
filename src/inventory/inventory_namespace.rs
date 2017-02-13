use std::path::PathBuf;

use serde_json::Value as JsonValue;

pub struct InventoryNamespace {

	raw_data: JsonValue,
	source_path: PathBuf,

	identity_name: String,

}

impl InventoryNamespace {

	property_accessors! {
		ref identity_name: & str;
	}

}

// ex: noet ts=4 filetype=rust
