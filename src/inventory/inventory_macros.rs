#[ doc (hide) ]
pub mod imports {
	pub use ::inventory::inventory_macros::capitalise;
	pub use ::linked_hash_map::LinkedHashMap;
	pub use ::serde_yaml::from_value as from_yaml_value;
	pub use ::serde_yaml::Value as YamlValue;
	pub use ::serde_yaml::Value::Mapping as YamlMapping;
	pub use ::serde_yaml::Value::Sequence as YamlSequence;
	pub use ::serde_yaml::Value::String as YamlString;
	pub use ::std::error::Error;
}

#[ allow (unused) ]
pub fn capitalise (
	string: & str,
) -> String {

	let mut chars =
		string.chars ();

	match chars.next () {

		None =>
			String::new (),

		Some (first_char) =>
			first_char.to_uppercase ().collect::<String> ()
				+ chars.as_str (),

	}

}

macro_rules! inventory_parser {

	(
		name $name:ident ;
		data $data:ident ;
		$( $rest:tt ) *
	) => {

		inventory_parser_declarations! {
			$( $rest ) *
		}

		{

			let mapping =
				$data.as_mapping ().ok_or_else (||
					format! (
						"{} must be a dictionary",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)))
				) ?;

			let raw_identity =
				mapping.get (
					& YamlValue::String ("identity".to_string ()),
				).ok_or_else (||
					format! (
						"{} must contain an 'identity' section",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)))
				) ?.as_mapping ().ok_or_else (||
					format! (
						"{} 'identity' must be a dictionary",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)))
				) ?;

			let identity_type =
				raw_identity.get (
					& YamlValue::String ("type".to_string ()),
				).ok_or_else (||
					format! (
						"{} 'identity' must contain 'type'",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)))
				) ?.as_str ().ok_or_else (||
					format! (
						"{} 'identity.type' must be a string",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)))
				) ?;

			if identity_type != stringify! ($name) {

				return Err (
					format! (
						"{} 'identity.type' must be '{}'",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($name)));

			}

			inventory_parser_logic! {
				name $name;
				mapping mapping;
				$( $rest ) *
			}

		}

	};

}

macro_rules! inventory_parser_declarations {

	(
		section $name:ident {
			$( $content:tt ) *
		}
		$( $rest:tt ) *
	) => {

		inventory_parser_section_declarations! {
			$( $content ) *
		}

		inventory_parser_declarations! {
			$( $rest ) *
		}

	};

	() => {};

}

macro_rules! inventory_parser_section_declarations {

	(
		req $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: $value_type;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	(
		opt $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: Option <$value_type>;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	(
		vec $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: Vec <$value_type>;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	(
		map $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: HashMap <String, $value_type>;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	() => {};

}

macro_rules! inventory_parser_logic {

	(
		name $name:ident ;
		mapping $mapping:ident ;
		section $section_name:ident {
			$( $section_content:tt ) *
		}
		$( $rest:tt ) *
	) => {

		{

			use ::inventory::inventory_macros::imports::*;

			let section_mapping =
				$mapping.get (
					& YamlValue::String (
						stringify! ($section_name).to_string (),
					),
				).ok_or_else (||
					format! (
						"{} must contain section '{}'",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($section_name))
				) ?.as_mapping ().ok_or_else (||
					format! (
						"{} section '{}' must be a dictionary",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($section_name))
				) ?;

			inventory_parser_section_logic! {
				section_name $section_name;
				section_mapping section_mapping;
				$( $section_content ) *
			}

		}

		inventory_parser_logic! {
			name $name;
			mapping $mapping;
			$( $rest ) *
		}

	};

	(
		name $name:ident ;
		mapping $mapping:ident ;
	) => {};

}

macro_rules! inventory_parser_section_logic {

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		req $name:ident : String = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			$section_mapping.get (
				& YamlValue::String (
					$key.to_string (),
				),
			).ok_or_else (||
				format! (
					"{} section '{}' must contain '{}'",
					::inventory::inventory_macros::capitalise (
						stringify! ($name)),
					stringify! ($section_name),
					stringify! ($key)),
			) ?.as_str ().ok_or_else (||
				format! (
					"{} value '{}.{}' must be a string",
					::inventory::inventory_macros::capitalise (
						stringify! ($name)),
					stringify! ($section_name),
					stringify! ($key)),
			) ?.to_owned ();

		inventory_parser_section_logic! {

			section_name $section_name;
			section_mapping $section_mapping;

			$( $rest ) *

		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		opt $name:ident : String = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			$section_mapping.get (
				& YamlValue::String (
					$key.to_string (),
				),
			).map (|value|
				value.as_str ().ok_or_else (||
					format! (
						"{} value '{}.{}' must be a string (if present)",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($section_name),
						stringify! ($key))
				).map (|value|
					Some (value),
				)
			).unwrap_or (
				Ok (None),
			) ?.map (|value|
				value.to_owned (),
			);

		inventory_parser_section_logic! {

			section_name $section_name;
			section_mapping $section_mapping;

			$( $rest ) *

		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		vec $name:ident : String = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			Vec::new ();

		inventory_parser_section_logic! {
			section_name $section_name;
			section_mapping $section_mapping;
			$( $rest ) *
		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		opt $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			$section_mapping.get (
				& YamlValue::String (
					$key.to_string (),
				),
			).map (|value|
				::serde_yaml::from_value (
					value.clone (),
				).map_err (|error|
					format! (
						"{} value '{}.{}' must be a {} (if present)",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($section_name),
						stringify! ($key),
						stringify! ($value_type),
					),
				),
			).unwrap_or (
				Ok (None),
			) ?;

		inventory_parser_section_logic! {

			section_name $section_name;
			section_mapping $section_mapping;

			$( $rest ) *

		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		vec $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			$section_mapping.get (
				& YamlValue::String (
					$key.to_string (),
				),
			).unwrap_or (
				& YamlSequence (
					Vec::new (),
				),
			).as_sequence ().ok_or_else (||
				format! (
					"{} value '{}.{}' must be a list",
					capitalise (stringify! ($name)),
					stringify! ($section_name),
					stringify! ($key)),
			) ?.clone ().into_iter ().map (|value|
				from_yaml_value::<$value_type> (
					value,
				).map_err (|error|
					format! (
						"{} value '{}.{}' members must be {}: {}",
						::inventory::inventory_macros::capitalise (
							stringify! ($name)),
						stringify! ($section_name),
						stringify! ($key),
						stringify! ($value_type),
						error.description (),
					),
				),
			).collect::<Result <Vec <$value_type>, String>> () ?;

		inventory_parser_section_logic! {

			section_name $section_name;
			section_mapping $section_mapping;

			$( $rest ) *

		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
		map $name:ident : $value_type:ty = $key:expr ;
		$( $rest:tt ) *
	) => {

		$name =
			$section_mapping.get (
				& YamlValue::String (
					$key.to_string (),
				),
			).unwrap_or (
				& YamlMapping (
					LinkedHashMap::new (),
				),
			).as_mapping ().ok_or_else (||
				format! (
					"{} value '{}.{}' must be a dictionary",
					capitalise (stringify! ($name)),
					stringify! ($section_name),
					stringify! ($key)),
			) ?.clone ().into_iter ().map (|(key, value)|
				Ok ((
					match key {
						YamlString (value) => value.to_owned (),
						_ => panic! (),
					},
					from_yaml_value::<$value_type> (
						value,
					).map_err (|error|
						format! (
							"{} value '{}.{}' members must be {}: {}",
							capitalise (stringify! ($name)),
							stringify! ($section_name),
							stringify! ($key),
							stringify! ($value_type),
							error.description (),
						),
					) ?,
				)),
			).collect::<Result <HashMap <String, $value_type>, String>> () ?;

		inventory_parser_section_logic! {

			section_name $section_name;
			section_mapping $section_mapping;

			$( $rest ) *

		}

	};

	(
		section_name $section_name:ident ;
		section_mapping $section_mapping:ident ;
	) => {};

}

// ex: noet ts=4 filetype=rust
