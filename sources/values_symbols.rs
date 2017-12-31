

use super::runtime::exports::*;
use super::values_strings::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Symbol;
	pub use super::{symbol_new, symbol_clone_str, symbol_clone_characters};
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Symbol ( StdRc<StdBox<str>> );


impl Symbol {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	pub fn is_self (&self, other : &Symbol) -> (bool) {
		let self_0 = self.0.as_ref ();
		let other_0 = other.0.as_ref ();
		ptr::eq (self_0, other_0) && (self_0 == other_0)
	}
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		self.0.clone ()
	}
}


impl String for Symbol {
	
	#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
	fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
}




#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn symbol_new (string : StdString) -> (Symbol) {
	Symbol (StdRc::new (string.into_boxed_str ()))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn symbol_clone_str (string : &str) -> (Symbol) {
	symbol_new (StdString::from (string))
}

#[ cfg_attr ( feature = "scheme_inline_always", inline ) ]
pub fn symbol_clone_characters (characters : &[char]) -> (Symbol) {
	symbol_new (unicode_utf8_chars_clone (characters))
}

