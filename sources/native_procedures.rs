

use super::errors::exports::*;
use super::evaluator::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::ProcedureNative;
	pub use super::ProcedureNativeInternals;
	
	pub use super::ProcedureNative0;
	pub use super::ProcedureNative1;
	pub use super::ProcedureNative2;
	pub use super::ProcedureNative3;
	pub use super::ProcedureNative4;
	pub use super::ProcedureNative5;
	pub use super::ProcedureNativeN;
	
	pub use super::ProcedureNative0E;
	pub use super::ProcedureNative1E;
	pub use super::ProcedureNative2E;
	pub use super::ProcedureNative3E;
	pub use super::ProcedureNative4E;
	pub use super::ProcedureNative5E;
	pub use super::ProcedureNativeNE;
	
	pub use super::ProcedureNativeV;
	
	pub use super::ProcedureNativeFn0;
	pub use super::ProcedureNativeFn1;
	pub use super::ProcedureNativeFn2;
	pub use super::ProcedureNativeFn3;
	pub use super::ProcedureNativeFn4;
	pub use super::ProcedureNativeFn5;
	pub use super::ProcedureNativeFnN;
	
	pub use super::ProcedureNativeFn0E;
	pub use super::ProcedureNativeFn1E;
	pub use super::ProcedureNativeFn2E;
	pub use super::ProcedureNativeFn3E;
	pub use super::ProcedureNativeFn4E;
	pub use super::ProcedureNativeFn5E;
	pub use super::ProcedureNativeFnNE;
	
	pub use super::ProcedureNativeFnV;
	
	pub use super::super::conversions::{
			procedure_native_0, procedure_native_0e,
			procedure_native_1, procedure_native_1e,
			procedure_native_2, procedure_native_2e,
			procedure_native_3, procedure_native_3e,
			procedure_native_4, procedure_native_4e,
			procedure_native_5, procedure_native_5e,
			procedure_native_n, procedure_native_ne,
			procedure_native_v,
		};
	
}




#[ derive (Clone) ]
pub struct ProcedureNative0 (pub ProcedureNativeFn0);

#[ derive (Clone) ]
pub struct ProcedureNative1 (pub ProcedureNativeFn1);

#[ derive (Clone) ]
pub struct ProcedureNative2 (pub ProcedureNativeFn2);

#[ derive (Clone) ]
pub struct ProcedureNative3 (pub ProcedureNativeFn3);

#[ derive (Clone) ]
pub struct ProcedureNative4 (pub ProcedureNativeFn4);

#[ derive (Clone) ]
pub struct ProcedureNative5 (pub ProcedureNativeFn5);

#[ derive (Clone) ]
pub struct ProcedureNativeN (pub ProcedureNativeFnN);


#[ derive (Clone) ]
pub struct ProcedureNative0E (pub ProcedureNativeFn0E);

#[ derive (Clone) ]
pub struct ProcedureNative1E (pub ProcedureNativeFn1E);

#[ derive (Clone) ]
pub struct ProcedureNative2E (pub ProcedureNativeFn2E);

#[ derive (Clone) ]
pub struct ProcedureNative3E (pub ProcedureNativeFn3E);

#[ derive (Clone) ]
pub struct ProcedureNative4E (pub ProcedureNativeFn4E);

#[ derive (Clone) ]
pub struct ProcedureNative5E (pub ProcedureNativeFn5E);

#[ derive (Clone) ]
pub struct ProcedureNativeNE (pub ProcedureNativeFnNE);


#[ derive (Clone) ]
pub struct ProcedureNativeV (pub ProcedureNativeFnV);




pub type ProcedureNativeFn0 = fn () -> (Outcome<Value>);
pub type ProcedureNativeFn1 = fn (&Value) -> (Outcome<Value>);
pub type ProcedureNativeFn2 = fn (&Value, &Value) -> (Outcome<Value>);
pub type ProcedureNativeFn3 = fn (&Value, &Value, &Value) -> (Outcome<Value>);
pub type ProcedureNativeFn4 = fn (&Value, &Value, &Value, &Value) -> (Outcome<Value>);
pub type ProcedureNativeFn5 = fn (&Value, &Value, &Value, &Value, &Value) -> (Outcome<Value>);
pub type ProcedureNativeFnN = fn (&[&Value]) -> (Outcome<Value>);

pub type ProcedureNativeFn0E = fn (&mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFn1E = fn (&Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFn2E = fn (&Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFn3E = fn (&Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFn4E = fn (&Value, &Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFn5E = fn (&Value, &Value, &Value, &Value, &Value, &mut EvaluatorContext) -> (Outcome<Value>);
pub type ProcedureNativeFnNE = fn (&[&Value], &mut EvaluatorContext) -> (Outcome<Value>);

pub type ProcedureNativeFnV = fn (arguments : usize) -> (Outcome<ProcedureNativeInternals>);




#[ derive (Clone, Debug) ]
pub struct ProcedureNative ( StdRc<ProcedureNativeInternals> );


#[ derive (Clone, Debug) ]
pub enum ProcedureNativeInternals {
	
	Native0 (ProcedureNative0),
	Native1 (ProcedureNative1),
	Native2 (ProcedureNative2),
	Native3 (ProcedureNative3),
	Native4 (ProcedureNative4),
	Native5 (ProcedureNative5),
	NativeN (ProcedureNativeN),
	
	Native0E (ProcedureNative0E),
	Native1E (ProcedureNative1E),
	Native2E (ProcedureNative2E),
	Native3E (ProcedureNative3E),
	Native4E (ProcedureNative4E),
	Native5E (ProcedureNative5E),
	NativeNE (ProcedureNativeNE),
	
	NativeV (ProcedureNativeV),
	
}


impl ProcedureNative {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn new (internals : ProcedureNativeInternals) -> (ProcedureNative) {
		return ProcedureNative (StdRc::new (internals));
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_ref (&self) -> (&ProcedureNativeInternals) {
		return StdRc::as_ref (&self.0);
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn internals_into (self) -> (ProcedureNativeInternals) {
		let self_0 = self.internals_ref ();
		return self_0.clone ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		let self_0 = self.internals_ref ();
		return self_0.symbol ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		let self_0 = self.internals_ref ();
		return self_0.handle ();
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn is_self (&self, other : &ProcedureNative) -> (bool) {
		StdRc::ptr_eq (&self.0, &other.0) || Handle::eq (&self.handle (), &other.handle ())
	}
}




impl ProcedureNativeInternals {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		match *self {
			ProcedureNativeInternals::Native0 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native1 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native2 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native3 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native4 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native5 (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::NativeN (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native0E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native1E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native2E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native3E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native4E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::Native5E (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::NativeNE (ref native) =>
				return native.symbol (),
			ProcedureNativeInternals::NativeV (ref native) =>
				return native.symbol (),
		}
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}




impl ProcedureNative0 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative1 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative2 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative3 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative4 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative5 {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNativeN {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}


impl ProcedureNative0E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative1E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative2E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative3E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative4E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNative5E {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

impl ProcedureNativeNE {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}


impl ProcedureNativeV {
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn symbol (&self) -> (BacktraceSymbol) {
		return BacktraceSymbol::new (unsafe { mem::transmute (self.0) });
	}
	
	#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
	pub fn handle (&self) -> (Handle) {
		return self.symbol () .handle ();
	}
}

