use anyhow;
use sha2::{Digest, Sha512};

pub mod simple_file;

pub type Hash = [u8; 64];

///Database witch does store a map from a hash to a matrix media url,
///to avaid duplicated uploads of the same file
pub trait Database {
	fn get(&self, hash: &Hash) -> Option<String>;
	fn add(&self, hash: Hash, url: String) -> anyhow::Result<()>;
}

pub fn hash(value: &Vec<u8>) -> Hash {
	let mut hasher = Sha512::new();
	hasher.update(value);
	hasher.finalize().into()
}
