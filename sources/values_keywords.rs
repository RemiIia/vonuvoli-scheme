

use super::runtime::exports::*;

#[ cfg ( feature = "vonuvoli_values_string" ) ]
use super::values_strings::exports::*;

use super::prelude::*;




pub mod exports {
	pub use super::Keyword;
	pub use super::{keyword_new, keyword_clone_str, keyword_clone_characters};
}




#[ derive (Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub struct Keyword ( StdRc<StdBox<str>> );


impl Keyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn from_rc (rc : StdRc<StdBox<str>>) -> (Keyword) {
		Keyword (rc)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn clone_rc (rc : &StdRc<StdBox<str>>) -> (Keyword) {
		Keyword::from_rc (StdRc::clone (rc))
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &Keyword) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || StdRc::eq (&self.0, &other.0)
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn string_rc_clone (&self) -> (StdRc<StdBox<str>>) {
		self.0.clone ()
	}
}


#[ cfg ( feature = "vonuvoli_values_string" ) ]
impl String for Keyword {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	fn string_as_str (&self) -> (&str) {
		self.0.as_ref ()
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_new (string : StdString) -> (Keyword) {
	Keyword (StdRc::new (string.into_boxed_str ()))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_clone_str (string : &str) -> (Keyword) {
	keyword_new (StdString::from (string))
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn keyword_clone_characters (characters : &[char]) -> (Keyword) {
	keyword_new (unicode_utf8_chars_clone_string (characters))
}

