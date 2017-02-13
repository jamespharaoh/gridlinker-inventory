#[ macro_use ]
mod inventory_macros;

mod inventory;
mod inventory_resource;
mod inventory_class;
mod inventory_group;
mod inventory_namespace;

pub use self::inventory_resource::*;
pub use self::inventory_class::*;
pub use self::inventory_group::*;
pub use self::inventory_namespace::*;
pub use self::inventory::*;

// ex: noet ts=4 filetype=rust
