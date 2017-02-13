macro_rules! property_accessors {

	(

		ref $name:ident : $return_type:ty ;

		$( $rest:tt ) *

	) => {

		pub fn $name (
			& self,
		) -> $return_type {
			& self.$name
		}

		property_accessors! {
			$( $rest ) *
		}

	};

	() => {};

}

// ex: noet ts=4 filetype=rust
