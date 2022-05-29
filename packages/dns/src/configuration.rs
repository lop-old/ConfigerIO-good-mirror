
use std::fs::read_to_string;
use std::collections::BTreeMap as Map;
use serde::{ Serialize, Deserialize };



#[derive(Debug, Serialize, Deserialize)]
pub struct DomainHost {
	pub ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
	pub internal: Map<String, DomainHost>,
	pub external: Map<String, DomainHost>,
}



impl Configuration {

	pub fn load(file: String) -> Self {
		let contents = read_to_string(file.clone())
			.unwrap_or_else(|e| panic!("Failed to read configer config file: {} {}", file, e));
		serde_json::from_str(&contents)
			.unwrap_or_else(|e| panic!("Failed to parse configer config file: {} {}", file, e))
	}

	pub fn get_internal_hosts(&self) -> Vec<String> {
		let mut array: Vec<String> = Vec::new();
		for key in self.internal.keys() {
			array.push(key.clone());
		}
		array
	}

	pub fn get_external_hosts(&self) -> Vec<String> {
		let mut array: Vec<String> = Vec::new();
		for key in self.external.keys() {
			array.push(key.clone());
		}
		array
	}

}

pub fn get_hostnames(array: &Map<String, DomainHost>) -> Vec<String> {
	let mut result: Vec<String> = Vec::new();
	for (hostname, _) in array {
		result.push(hostname.clone());
	}
	result
}
