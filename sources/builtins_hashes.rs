

use super::errors::exports::*;
use super::hashes::exports::*;

#[ allow (unused_imports) ]
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::{
			hash_value_with_default,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
	pub use super::{
			hash_value_with_siphash_seeded,
			hash_value_with_siphash_unseeded,
			coerce_siphash_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
	pub use super::{
			hash_value_with_seahash_seeded,
			hash_value_with_seahash_unseeded,
			coerce_seahash_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	pub use super::{
			hash_value_with_blake2b_seeded,
			hash_value_with_blake2b_unseeded,
			coerce_blake2b_seed,
		};
	
	#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
	pub use super::{
			hash_value_with_blake2s_seeded,
			hash_value_with_blake2s_unseeded,
			coerce_blake2s_seed,
		};
	
	pub use super::super::hashes::{
			HashMode,
		};
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_default <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	#[ cfg ( feature = "lazy_static" ) ]
	let hasher = hash::BuildHasher::build_hasher (RANDOM_STATE.deref ());
	#[ cfg ( not ( feature = "lazy_static" ) ) ]
	let hasher = hash_map::DefaultHasher::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "lazy_static" ) ]
lazy_static! {
	static ref RANDOM_STATE : hash_map::RandomState = hash_map::RandomState::new ();
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_siphash_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&(u64, u64)>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = if let Some (seed) = seed {
		let &(seed_1, seed_2) = if let Some (seed) = seed {
			seed
		} else {
			SIPHASH_DEFAULT_SEED.deref ()
		};
		ext::siphasher::sip::SipHasher24::new_with_keys (seed_1, seed_2)
	} else {
		ext::siphasher::sip::SipHasher24::new ()
	};
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_siphash_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = ext::siphasher::sip::SipHasher24::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn coerce_siphash_seed (value : &Value) -> (Outcome<Option<Option<(u64, u64)>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0x63e3aa97),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac))));
				},
				16 => {
					let mut seed : [u8; 16] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : (u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some (seed)));
				},
				_ =>
					fail! (0xf3ec9555),
			}
		},
		_ =>
			fail! (0x05b4c4e3),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_siphash" ) ]
lazy_static! {
	static ref SIPHASH_DEFAULT_SEED : (u64, u64) = {
			use super::externals::rand::Rng;
			let mut generator = try_or_panic_0! (ext::rand::os::OsRng::new (), 0xd048ae20, github_issue_new);
			(generator.next_u64 (), generator.next_u64 ())
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_seahash_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, seed : Option<Option<&(u64, u64, u64, u64)>>, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = if let Some (seed) = seed {
		let &(seed_1, seed_2, seed_3, seed_4) = if let Some (seed) = seed {
			seed
		} else {
			SEAHASH_DEFAULT_SEED.deref ()
		};
		ext::seahash::SeaHasher::with_seeds (seed_1, seed_2, seed_3, seed_4)
	} else {
		ext::seahash::SeaHasher::new ()
	};
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_seahash_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, mode : Option<HashMode>) -> (Outcome<u64>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let hasher = ext::seahash::SeaHasher::new ();
	return hash_value_with_hasher (value, hasher, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn coerce_seahash_seed (value : &Value) -> (Outcome<Option<Option<(u64, u64, u64, u64)>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed = value.value ();
			let seed = seed as u64;
			succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac, seed ^ 0x65e2b15736aae9df, seed ^ 0x9bc88f9627d32f76))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = try! (value.bytes_ref ());
			let bytes = bytes.bytes_as_slice ();
			match bytes.len () {
				0 =>
					fail! (0x4d9bb47d),
				8 => {
					let mut seed : [u8; 8] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : u64 = unsafe { mem::transmute (seed) };
					succeed! (Some (Some ((seed, seed ^ 0x373c408e42ea64ac, seed ^ 0x65e2b15736aae9df, seed ^ 0x9bc88f9627d32f76))));
				},
				32 => {
					let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
					seed.copy_from_slice (bytes);
					let seed : (u64, u64, u64, u64) = unsafe { mem::transmute (seed) };
					succeed! (Some (Some (seed)));
				},
				_ =>
					fail! (0xdb8aabed),
			}
		},
		_ =>
			fail! (0x3ce9d745),
	}
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_seahash" ) ]
lazy_static! {
	static ref SEAHASH_DEFAULT_SEED : (u64, u64, u64, u64) = {
			use super::externals::rand::Rng;
			let mut generator = try_or_panic_0! (ext::rand::os::OsRng::new (), 0xf15c6b9a, github_issue_new);
			(generator.next_u64 (), generator.next_u64 (), generator.next_u64 (), generator.next_u64 ())
		};
}




#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2b_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, seed : Option<Option<&[u8]>>, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		if let Some (seed) = seed {
			Some (seed)
		} else {
			let seed = BLAKE2B_DEFAULT_SEED.deref ();
			Some (&seed[..])
		}
	} else {
		None
	};
	return hash_value_with_blake2b (value, bits, seed, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2b_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	return hash_value_with_blake2b (value, bits, None, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn coerce_blake2b_seed (value : &Value) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	return coerce_blake2_seed (value, 64);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
lazy_static! {
	static ref BLAKE2B_DEFAULT_SEED : [u8; 64] = {
			use super::externals::rand::Rng;
			let mut seed : [u8; 64] = unsafe { mem::uninitialized () };
			let mut generator = try_or_panic_0! (ext::rand::os::OsRng::new (), 0x942cef13, github_issue_new);
			generator.fill_bytes (&mut seed);
			seed
		};
}


#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2s_seeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, seed : Option<Option<&[u8]>>, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	let seed = if let Some (seed) = seed {
		if let Some (seed) = seed {
			Some (seed)
		} else {
			let seed = BLAKE2S_DEFAULT_SEED.deref ();
			Some (&seed[..])
		}
	} else {
		None
	};
	return hash_value_with_blake2s (value, bits, seed, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn hash_value_with_blake2s_unseeded <Value : HashValue, ValueRef : StdAsRef<Value>> (value : ValueRef, bits : usize, mode : Option<HashMode>) -> (Outcome<StdBox<[u8]>>) {
	let mode = mode.unwrap_or (DEFAULT_HASH_MODE);
	return hash_value_with_blake2s (value, bits, None, mode);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn coerce_blake2s_seed (value : &Value) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	return coerce_blake2_seed (value, 32);
}

#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
lazy_static! {
	static ref BLAKE2S_DEFAULT_SEED : [u8; 32] = {
			use super::externals::rand::Rng;
			let mut seed : [u8; 32] = unsafe { mem::uninitialized () };
			let mut generator = try_or_panic_0! (ext::rand::os::OsRng::new (), 0x01931b92, github_issue_new);
			generator.fill_bytes (&mut seed);
			seed
		};
}


#[ cfg ( feature = "vonuvoli_builtins_hashes_blake2" ) ]
#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
fn coerce_blake2_seed (value : &Value, max_size : usize) -> (Outcome<Option<Option<GenericRef<[u8]>>>>) {
	match value.kind_match_as_ref () {
		ValueKindMatchAsRef::Boolean (value) => {
			if value.value () {
				succeed! (Some (None));
			} else {
				succeed! (None);
			}
		},
		ValueKindMatchAsRef::NumberInteger (value) => {
			let seed : &[u8; 8] = unsafe { mem::transmute (value) };
			succeed! (Some (Some (GenericRef::Immutable (seed))));
		},
		_ =>
			(),
	}
	match value.class_match_as_ref () {
		#[ cfg ( feature = "vonuvoli_values_bytes" ) ]
		ValueClassMatchAsRef::Bytes (value) => {
			let bytes = try! (value.bytes_ref ());
			{
				let bytes = bytes.bytes_as_slice ();
				if bytes.is_empty () {
					succeed! (None);
				}
				if bytes.len () > max_size {
					fail! (0xe1446d5a);
				}
			}
			succeed! (Some (Some (bytes.into_generic_ref ())));
		},
		_ =>
			fail! (0x1f1571ee),
	}
}




const DEFAULT_HASH_MODE : HashMode = HashMode::ValuesCoerceMutable;

