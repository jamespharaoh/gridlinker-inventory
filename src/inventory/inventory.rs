use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::Arc;

use inventory::*;
use settings::*;

pub struct Inventory {

	settings: Arc <Settings>,

	classes_list: Vec <Arc <InventoryClass>>,
	classes_map: HashMap <String, Arc <InventoryClass>>,

	namespaces_list: Vec <Arc <InventoryNamespace>>,
	namespaces_map: HashMap <String, Arc <InventoryNamespace>>,

}

impl Inventory {

	pub fn load (
		settings: Arc <Settings>,
	) -> Result <Inventory, String> {

		let classes_list =
			Self::load_classes (
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
			Self::load_namespaces (
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

			classes_list: classes_list,
			classes_map: classes_map,

			namespaces_list: namespaces_list,
			namespaces_map: namespaces_map,

		})

	}

	fn load_classes (
		settings: & Settings,
		classes_path: & Path,
	) -> Result <Vec <Arc <InventoryClass>>, String> {

		let mut classes_list: Vec <Arc <InventoryClass>> =
			Vec::new ();

		Self::load_classes_impl (
			settings,
			classes_path,
			& mut classes_list,
		) ?;

		Ok (classes_list)

	}

	fn load_classes_impl (
		settings: & Settings,
		classes_path: & Path,
		classes_list: & mut Vec <Arc <InventoryClass>>,
	) -> Result <(), String> {

		for dir_entry_result
		in classes_path.read_dir (
		).map_err (|error|
			format! (
				"Error reading classes directory {}: {}",
				classes_path.to_string_lossy (),
				error.description ())
		) ? {

			let dir_entry =
				dir_entry_result.map_err (|error|
					format! (
						"Error reading classes directory {}: {}",
						classes_path.to_string_lossy (),
						error.description ())
				) ?;

			let dir_entry_path =
				dir_entry.path ();

			let dir_entry_metadata =
				dir_entry_path.metadata (
				).map_err (|error|
					format! (
						"Error reading classes directory {}: {}",
						dir_entry_path.to_string_lossy (),
						error.description ())
				) ?;

			if dir_entry_metadata.is_file () {

				classes_list.push (
					Self::load_class (
						settings,
						& dir_entry_path,
					) ?
				);

			} else if dir_entry_metadata.is_dir () {

				Self::load_classes_impl (
					settings,
					& dir_entry_path,
					classes_list,
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

	fn load_class (
		settings: & Settings,
		class_path: & Path,
	) -> Result <Arc <InventoryClass>, String> {

		Err ("TODO".to_string ())

	}

	fn load_namespaces (
		settings: & Settings,
		namespaces_path: & Path,
	) -> Result <Vec <Arc <InventoryNamespace>>, String> {

		let mut namespaces_list: Vec <Arc <InventoryNamespace>> =
			Vec::new ();

		Self::load_namespaces_impl (
			settings,
			namespaces_path,
			& mut namespaces_list,
		) ?;

		Ok (namespaces_list)

	}

	fn load_namespaces_impl (
		settings: & Settings,
		namespaces_path: & Path,
		namespaces_list: & mut Vec <Arc <InventoryNamespace>>,
	) -> Result <(), String> {

		for dir_entry_result
		in namespaces_path.read_dir (
		).map_err (|error|
			format! (
				"Error reading namespaces directory {}: {}",
				namespaces_path.to_string_lossy (),
				error.description ())
		) ? {

			let dir_entry =
				dir_entry_result.map_err (|error|
					format! (
						"Error reading namespaces directory {}: {}",
						namespaces_path.to_string_lossy (),
						error.description ())
				) ?;

			let dir_entry_path =
				dir_entry.path ();

			let dir_entry_metadata =
				dir_entry_path.metadata (
				).map_err (|error|
					format! (
						"Error reading namespaces directory {}: {}",
						dir_entry_path.to_string_lossy (),
						error.description ())
				) ?;

			if dir_entry_metadata.is_file () {

				namespaces_list.push (
					Self::load_namespace (
						settings,
						& dir_entry_path,
					) ?
				);

			} else if dir_entry_metadata.is_dir () {

				Self::load_namespaces_impl (
					settings,
					& dir_entry_path,
					namespaces_list,
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

	fn load_namespace (
		settings: & Settings,
		namespace_path: & Path,
	) -> Result <Arc <InventoryNamespace>, String> {

		Err ("TODO".to_owned ())

	}

}

// ex: noet ts=4 filetype=rust
