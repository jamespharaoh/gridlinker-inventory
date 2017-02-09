#[ derive (Serialize, Debug, Deserialize) ]
pub struct EtcdResponse {

	#[ serde (rename = "action") ]
	pub action: String,

	#[ serde (rename = "node") ]
	pub node: EtcdNode,

}

#[ derive (Serialize, Debug, Deserialize) ]
pub struct EtcdNode {

	#[ serde (rename = "key") ]
	pub key: String,

	#[ serde (rename = "value") ]
	pub value: Option <String>,

	#[ serde (rename = "dir", default = "false_default") ]
	pub dir: bool,

	#[ serde (rename = "nodes", default = "empty_vec_default") ]
	pub nodes: Vec <EtcdNode>,

	#[ serde (rename = "modifiedIndex") ]
	pub modified_index: u64,

	#[ serde (rename = "createdIndex") ]
	pub created_index: u64,

}

fn empty_vec_default <Type> () -> Vec <Type> { Vec::new () }
fn false_default () -> bool { false }

// ex: noet ts=4 filetype=rust
