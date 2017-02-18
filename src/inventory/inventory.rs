use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

use serde_yaml;
use serde_yaml::Value as YamlValue;

use inventory::*;
use settings::*;

pub struct Inventory {

	settings: Arc <Settings>,

	project: Arc <InventoryProject>,

	classes_list: Vec <Arc <InventoryClass>>,
	classes_map: HashMap <String, Arc <InventoryClass>>,

	namespaces_list: Vec <Arc <InventoryNamespace>>,
	namespaces_map: HashMap <String, Arc <InventoryNamespace>>,

}

impl Inventory {

	property_accessors! {

		ref project: & Arc <InventoryProject>;

		ref classes_list: & Vec <Arc <InventoryClass>>;
		ref classes_map: & HashMap <String, Arc <InventoryClass>>;

		ref namespaces_list: & Vec <Arc <InventoryNamespace>>;
		ref namespaces_map: & HashMap <String, Arc <InventoryNamespace>>;

	}

	pub fn load (
		settings: Arc <Settings>,
	) -> Result <Inventory, String> {

		let project =
			Self::load_file (
				& InventoryProject::new,
				& settings,
				& settings.general.project_data.join ("project"),
			) ?;

		let classes_list =
			Self::load_files (
				& InventoryClass::new,
				"classes",
				& settings,
				& settings.general.project_data.join ("classes"),
			) ?;

		let classes_map =
			classes_list.iter ().map (|class|
				(
					class.identity_name ().to_owned (),
					class.clone (),
				)
			).collect ();

		let namespaces_list =
			Self::load_files (
				& InventoryNamespace::new,
				"namespaces",
				& settings,
				& settings.general.project_data.join ("namespaces"),
			) ?;

		let namespaces_map =
			namespaces_list.iter ().map (|namespace|
				(
					namespace.identity_name ().to_owned (),
					namespace.clone (),
				)
			).collect ();

		Ok (Inventory {

			settings: settings,

			project: project,

			classes_list: classes_list,
			classes_map: classes_map,

			namespaces_list: namespaces_list,
			namespaces_map: namespaces_map,

		})

	}

	fn load_files <Type> (
		loader: & Fn (YamlValue) -> Result <Type, String>,
		name_plural: & str,
		settings: & Settings,
		root_path: & Path,
	) -> Result <Vec <Arc <Type>>, String> {

		let mut items: Vec <Arc <Type>> =
			Vec::new ();

		Self::load_files_impl (
			loader,
			name_plural,
			settings,
			root_path,
			& mut items,
		) ?;

		Ok (items)

	}

	fn load_files_impl <Type> (
		loader: & Fn (YamlValue) -> Result <Type, String>,
		name_plural: & str,
		settings: & Settings,
		path: & Path,
		items: & mut Vec <Arc <Type>>,
	) -> Result <(), String> {

		for dir_entry_result
		in path.read_dir (
		).map_err (|error|
			format! (
				"Error reading {} directory {}: {}",
				name_plural,
				path.to_string_lossy (),
				error.description ())
		) ? {

			let dir_entry =
				dir_entry_result.map_err (|error|
					format! (
						"Error reading {} directory {}: {}",
						name_plural,
						path.to_string_lossy (),
						error.description ())
				) ?;

			let dir_entry_path =
				dir_entry.path ();

			let dir_entry_metadata =
				dir_entry_path.metadata (
				).map_err (|error|
					format! (
						"Error reading {} directory {}: {}",
						name_plural,
						dir_entry_path.to_string_lossy (),
						error.description ())
				) ?;

			if dir_entry_metadata.is_file () {

				items.push (
					Self::load_file (
						loader,
						settings,
						& dir_entry_path,
					) ?
				);

			} else if dir_entry_metadata.is_dir () {

				Self::load_files_impl (
					loader,
					name_plural,
					settings,
					& dir_entry_path,
					items,
				) ?

			} else {

				return Err (
					format! (
						"Invalid file type {}: {:?}",
						dir_entry.path ().to_string_lossy (),
						dir_entry.file_type ()));

			}

		}

		Ok (())

	}

	fn load_file <Type> (
		loader: & Fn (YamlValue) -> Result <Type, String>,
		settings: & Settings,
		path: & Path,
	) -> Result <Arc <Type>, String> {

		let file =
			File::open (
				path,
			).map_err (|error|
				format! (
					"Error reading {}: {}",
					path.to_string_lossy (),
					error.description ())
			) ?;

		let raw_data =
			serde_yaml::from_reader (
				file,
			).map_err (|error|
				format! (
					"Error parsing {}: {}",
					path.to_string_lossy (),
					error.description ())
			) ?;

		let item =
			loader (
				raw_data,
			).map_err (|error|
				format! (
					"Error loading {}: {}",
					path.to_string_lossy (),
					error)
			) ?;

		Ok (Arc::new (item))

	}

}

// ex: noet ts=4 filetype=rust
