

use super::prelude::*;




pub mod exports {
	
	pub use super::SyntaxPrimitiveV;
	pub use super::SyntaxPrimitive;
	
}




include! ("./macros_primitives.in");




#[ derive ( Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash ) ] // OK
#[ cfg_attr ( feature = "vonuvoli_fmt_debug", derive ( Debug ) ) ] // OK
pub enum SyntaxPrimitive {
	
	PrimitiveV ( SyntaxPrimitiveV ),
	
	Auxiliary,
	
	Unimplemented,
	Unsupported,
	Reserved,
	
}


def_primitives_enum! (SyntaxPrimitiveV, (syntax, v), {
	
	Quote,
	QuasiQuote,
	UnQuote,
	UnQuoteSplicing,
	
	Begin,
	And,
	Or,
	
	If,
	When,
	Unless,
	Cond,
	#[ cfg ( feature = "vonuvoli_builtins_comparisons" ) ]
	Case,
	
	Do,
	DoCond,
	While,
	WhileCond,
	Until,
	UntilCond,
	Loop,
	
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	Guard,
	#[ cfg ( feature = "vonuvoli_values_error" ) ]
	GuardCond,
	
	Locals,
	LetParallel,
	LetSequential,
	LetRecursiveParallel,
	LetRecursiveSequential,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	LetValuesParallel,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	LetValuesSequential,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	LetParameters,
	
	Define,
	ReDefine,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	DefineValues,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	ReDefineValues,
	
	Set,
	#[ cfg ( feature = "vonuvoli_values_values" ) ]
	SetValues,
	
	#[ cfg ( feature = "vonuvoli_expressions" ) ]
	#[ cfg ( feature = "vonuvoli_values_lambda" ) ]
	Lambda,
	
	#[ cfg ( feature = "vonuvoli_builtins_records" ) ]
	DefineRecord,
	
});




impl SyntaxPrimitive {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &SyntaxPrimitive) -> (bool) {
		*self == *other
	}
}

