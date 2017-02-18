use std::collections::HashMap;

use serde_yaml::Value as YamlValue;

#[ derive (Serialize, Deserialize) ]
pub struct InventoryDeveloper {

	#[ serde (rename = "name") ]
	pub name: String,

	#[ serde (rename = "email") ]
	pub email: String,

}

impl InventoryDeveloper {

	property_accessors! {
		ref name: & str;
		ref email: & str;
	}

}

#[ derive (Serialize, Deserialize) ]
pub struct InventoryCertificateDefaults {

	#[ serde (rename = "country") ]
	country_code: String,

	#[ serde (rename = "locality") ]
	locality: String,

	#[ serde (rename = "organization") ]
	organization: String,

}

#[ derive (Serialize, Deserialize) ]
pub struct InventoryResourceData {

	#[ serde (rename = "group") ]
	group_name: String,

	#[ serde (rename = "key") ]
	key_expression: String,

	#[ serde (rename = "section") ]
	section_name: Option <String>,

}

pub struct InventoryProject {

	raw_data: YamlValue,

	project_name: String,
	project_title: String,
	project_subject: String,

	project_short_name: String,
	project_short_title: String,

	project_script: String,
	project_repository: String,
	project_website: String,
	project_domain: String,

	project_developers: Vec <InventoryDeveloper>,

	gridlinker_environment: HashMap <String, String>,
	gridlinker_default_connections: Option <YamlValue>,

	certificate_defaults: Option <InventoryCertificateDefaults>,

	inventory_local_data: HashMap <String, String>,
	inventory_resource_data: HashMap <String, InventoryResourceData>,

}

impl InventoryProject {

	pub fn new (
		raw_data: YamlValue,
	) -> Result <InventoryProject, String> {

		inventory_parser! {

			name project;
			data raw_data;

			section project {

				req project_name: String = "name";
				req project_title: String = "title";
				req project_subject: String = "subject";

				req project_short_name: String = "short_name";
				req project_short_title: String = "short_title";

				req project_script: String = "script";
				req project_repository: String = "repository";
				req project_website: String = "website";
				req project_domain: String = "domain";

				vec project_developers: InventoryDeveloper = "developers";

			}

			section gridlinker {

				map gridlinker_environment: String = "environment";

				opt gridlinker_default_connections: YamlValue =
					"default_connections";

			}

			section certificate {

				opt certificate_defaults: InventoryCertificateDefaults =
					"defaults";

			}

			section inventory {

				map inventory_local_data: String = "local_data";

				map inventory_resource_data: InventoryResourceData =
					"resource_data";

			}

		}

		Ok (InventoryProject {

			raw_data: raw_data,

			project_name: project_name,
			project_title: project_title,
			project_subject: project_subject,

			project_short_name: project_short_name,
			project_short_title: project_short_title,

			project_script: project_script,
			project_repository: project_repository,
			project_website: project_website,
			project_domain: project_domain,

			project_developers: project_developers,

			gridlinker_environment: gridlinker_environment,
			gridlinker_default_connections: gridlinker_default_connections,

			certificate_defaults: certificate_defaults,

			inventory_local_data: inventory_local_data,
			inventory_resource_data: inventory_resource_data,

		})

	}

	property_accessors! {

		ref raw_data: & YamlValue;

		ref project_name: & str;
		ref project_title: & str;
		ref project_subject: & str;

		ref project_short_name: & str;
		ref project_short_title: & str;

		ref project_script: & str;
		ref project_repository: & str;
		ref project_website: & str;
		ref project_domain: & str;

		ref project_developers: & [InventoryDeveloper];

		ref gridlinker_environment: & HashMap <String, String>;
		ref gridlinker_default_connections: & Option <YamlValue>;

		ref certificate_defaults: & Option <InventoryCertificateDefaults>;

		ref inventory_local_data: & HashMap <String, String>;
		ref inventory_resource_data: & HashMap <String, InventoryResourceData>;

	}

}

// ex: noet ts=4 filetype=rust
