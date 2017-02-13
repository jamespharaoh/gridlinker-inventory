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
		req str $name:ident $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: String;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	(
		opt str $name:ident $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: Option <String>;

		inventory_parser_section_declarations! {
			$( $rest ) *
		}

	};

	(
		vec str $name:ident $key:expr ;
		$( $rest:tt ) *
	) => {

		let $name: Vec <String>;

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
		req str $name:ident $key:expr ;
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
		opt str $name:ident $key:expr ;
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
		vec str $name:ident $key:expr ;
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
	) => {};

}

// ex: noet ts=4 filetype=rust
