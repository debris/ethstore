//! Universaly unique identifier.
use std::str::FromStr;
use rustc_serialize::hex::{ToHex, FromHex};
use serde::{Deserialize, Serialize, Deserializer, Serializer, Error as SerdeError};
use serde::de::Visitor;
use super::Error;

/// Universaly unique identifier.
#[derive(Debug, PartialEq)]
pub struct UUID([u8; 16]);

impl From<[u8; 16]> for UUID {
	fn from(uuid: [u8; 16]) -> Self {
		UUID(uuid)
	}
}

impl<'a> Into<String> for &'a UUID {
	fn into(self) -> String {
		let d1 = &self.0[0..4];
		let d2 = &self.0[4..6];
		let d3 = &self.0[6..8];
		let d4 = &self.0[8..10];
		let d5 = &self.0[10..16];
		[d1, d2, d3, d4, d5].into_iter().map(|d| d.to_hex()).collect::<Vec<String>>().join("-")
	}
}

impl Into<String> for UUID {
	fn into(self) -> String {
		Into::into(&self)
	}
}

impl Into<[u8; 16]> for UUID {
	fn into(self) -> [u8; 16] {
		self.0
	}
}

fn clone_into(from: &str, into: &mut [u8]) -> Result<(), Error> {
	let from = try!(from.from_hex().map_err(|_| Error::InvalidUUID));

	if from.len() != into.len() {
		return Err(Error::InvalidUUID);
	}

	into.clone_from_slice(&from);
	Ok(())
}

impl FromStr for UUID {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parts: Vec<&str> = s.split("-").collect();

		if parts.len() != 5 { 
			return Err(Error::InvalidUUID); 
		}

		let mut uuid = [0u8; 16];

		try!(clone_into(parts[0], &mut uuid[0..4]));
		try!(clone_into(parts[1], &mut uuid[4..6]));
		try!(clone_into(parts[2], &mut uuid[6..8]));
		try!(clone_into(parts[3], &mut uuid[8..10]));
		try!(clone_into(parts[4], &mut uuid[10..16]));

		Ok(UUID(uuid))
	}
}

impl Serialize for UUID {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		let s: String = self.into();
		serializer.serialize_str(&s)
	}
}

impl Deserialize for UUID {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		deserializer.deserialize(UUIDVisitor)
	}
}

struct UUIDVisitor;

impl Visitor for UUIDVisitor {
	type Value = UUID;

	fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
		UUID::from_str(value).map_err(SerdeError::custom)
	}

	fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E> where E: SerdeError {
		self.visit_str(value.as_ref())
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;
	use super::UUID;

	#[test]
	fn uuid_from_str() {
		let uuid = UUID::from_str("3198bc9c-6672-5ab3-d995-4942343ae5b6").unwrap();
		assert_eq!(uuid, UUID::from([0x31, 0x98, 0xbc, 0x9c, 0x66, 0x72, 0x5a, 0xb3, 0xd9, 0x95, 0x49, 0x42, 0x34, 0x3a, 0xe5, 0xb6]));
	}

	#[test]
	fn uuid_from_and_to_str() {
		let from = "3198bc9c-6672-5ab3-d995-4942343ae5b6";
		let uuid = UUID::from_str(from).unwrap();
		let to: String = uuid.into();
		assert_eq!(from, &to);
	}
}