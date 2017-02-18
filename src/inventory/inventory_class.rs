use std::path::PathBuf;

use serde_yaml::Value as YamlValue;

pub struct InventoryClass {

	raw_data: YamlValue,

	identity_name: String,

	class_namespace: String,
	class_parent_namespace: Option <String>,
	class_groups: Vec <String>,

}

impl InventoryClass {

	pub fn new (
		raw_data: YamlValue,
	) -> Result <InventoryClass, String> {

		inventory_parser! {

			name class;
			data raw_data;

			section identity {
				req identity_name: String = "name";
			}

			section class {
				req class_namespace: String = "namespace";
				opt class_parent_namespace: String = "parent_namespace";
				vec class_groups: String = "groups";
			}

		}

		let class_groups: Vec <String> =
			Vec::new ();

		Ok (InventoryClass {

			raw_data: raw_data,

			identity_name: identity_name,

			class_namespace: class_namespace,
			class_parent_namespace: class_parent_namespace,
			class_groups: class_groups,

		})

	}

	property_accessors! {
		ref identity_name: & str;
		ref class_namespace: & str;
		ref class_parent_namespace: & Option <String>;
		ref class_groups: & [String];
	}

}

// ex: noet ts=4 filetype=rust
