use std::path::PathBuf;

use serde_yaml::Value as YamlValue;

pub struct InventoryNamespace {

	raw_data: YamlValue,

	identity_name: String,

}

impl InventoryNamespace {

	pub fn new (
		raw_data: YamlValue,
	) -> Result <InventoryNamespace, String> {

		inventory_parser! {

			name namespace;
			data raw_data;

			section identity {
				req identity_name: String = "name";
			}

		}

		Ok (InventoryNamespace {

			raw_data: raw_data,

			identity_name: identity_name,

		})

	}

	property_accessors! {
		ref identity_name: & str;
	}


}

// ex: noet ts=4 filetype=rust
