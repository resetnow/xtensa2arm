use std::vec::Vec;
use std::option::Option;

use rustc_serialize::json;

pub enum ObjectKind {
	Object { 
		/// Object data (from .data, .rodata or .bss)
		data: Vec<u8>,
		/// Whether or not this object was already
		/// retrieved from radare2
		read: bool
	},
	Function,
	Unknown
}

pub struct Object {
	address: u32,
	size: u32,
	name: String,
	kind: ObjectKind
}

#[derive(Default)]
pub struct ObjectStorage {
	objects: Vec<Object>
}

/// Stores objects extracted from ELF file
/// and provides methods to look up objects by
/// address
impl ObjectStorage {
	/// Takes Json output from radare2
	/// and populates object array
	pub fn from_json(mut self, json: json::Json) {
		let array = json.into_array().unwrap();

		for element in &array {
			let json_object = element.as_object().unwrap();
			let mut binary_object = Object { 
				address: 0,
				size: 0,
				name: Default::default(),
				kind: ObjectKind::Unknown
			};

		    for (key, value) in json_object.iter() {
    			match key as &str {
					"name" => {	binary_object.name = String::from(value.as_string().unwrap()) }
					"size" => { binary_object.size = value.as_u64().unwrap() as u32 }
					"type" => {
						binary_object.kind = match value.as_string().unwrap() {
							"OBJECT" => ObjectKind::Object { data: Vec::default(), read: false },
							"NOTYPE" => ObjectKind::Unknown,
							"FUNC"   => ObjectKind::Function,
							_ => panic!(),
						}
					}
					"vaddr" => { binary_object.address = value.as_u64().unwrap() as u32 }
					"paddr" |
					"demname" |
					"flagname" => {},
					_ => panic!(),
				}
    		}

			self.objects.push(binary_object)		
		}

		// TODO sort
	}
	
	/// Searches for an object at the provided address
	/// and returns a reference if found
	pub fn get_object<'a>(&'a self, address: u32) -> Option<&'a Object> {
		// TODO binary search
		// TODO offset result

		for object in &self.objects {
			if object.address == address {
				return Some(object)
			}
		}

		None
	}
}
