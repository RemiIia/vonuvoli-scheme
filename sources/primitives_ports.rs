

use super::builtins::exports::*;
use super::constants::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::ports::exports::*;
use super::primitives_procedures::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::PortPrimitive0;
	pub use super::PortPrimitive1;
	pub use super::PortPrimitive2;
	pub use super::PortPrimitive3;
	pub use super::PortPrimitive4;
	pub use super::PortPrimitive5;
	pub use super::PortPrimitiveN;
	pub use super::PortPrimitiveV;
	
	pub use super::port_primitive_0_evaluate;
	pub use super::port_primitive_1_evaluate;
	pub use super::port_primitive_2_evaluate;
	pub use super::port_primitive_3_evaluate;
	pub use super::port_primitive_4_evaluate;
	pub use super::port_primitive_5_evaluate;
	pub use super::port_primitive_n_evaluate;
	
	pub use super::port_primitive_v_alternative_0;
	pub use super::port_primitive_v_alternative_1;
	pub use super::port_primitive_v_alternative_2;
	pub use super::port_primitive_v_alternative_3;
	pub use super::port_primitive_v_alternative_4;
	pub use super::port_primitive_v_alternative_5;
	pub use super::port_primitive_v_alternative_n;
	
	pub use super::port_primitive_0_attributes;
	pub use super::port_primitive_1_attributes;
	pub use super::port_primitive_2_attributes;
	pub use super::port_primitive_3_attributes;
	pub use super::port_primitive_4_attributes;
	pub use super::port_primitive_5_attributes;
	pub use super::port_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive0 {
	
	OutputToBytes,
	OutputToString,
	
	CurrentInput,
	CurrentOutput,
	CurrentError,
	
	Eof,
	
	ByteReady,
	BytePeek,
	ByteRead,
	
	CharacterReady,
	CharacterPeek,
	CharacterRead,
	
	BytesReadCollect,
	BytesReadExtend,
	BytesReadCopy,
	BytesReadChunk,
	BytesReadLine,
	
	StringReadCollect,
	StringReadExtend,
	StringReadChunk,
	StringReadLine,
	
	ValueRead,
	
	NewLine,
	
	FlushOutput,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive1 {
	
	InputFromBytes,
	InputFromString,
	
	OutputToBytesFinalize,
	OutputToStringFinalize,
	
	OpenBinaryInput,
	OpenBinaryOutput,
	
	OpenTextualInput,
	OpenTextualOutput,
	
	IsInputOpen,
	IsOutputOpen,
	
	Close,
	CloseInput,
	CloseOutput,
	
	ByteReady,
	BytePeek,
	ByteRead,
	
	CharacterReady,
	CharacterPeek,
	CharacterRead,
	
	BytesReadCollect,
	BytesReadExtend,
	BytesReadCopy,
	BytesReadChunk,
	BytesReadLine,
	
	StringReadCollect,
	StringReadExtend,
	StringReadChunk,
	StringReadLine,
	
	ValueRead,
	
	ByteWrite,
	BytesWrite,
	
	CharacterWrite,
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
	NewLine,
	FlushOutput,
	
	FileExists,
	FileDelete,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive2 {
	
	CallAndClose,
	OpenBinaryInputThenCallAndClose,
	OpenBinaryOutputThenCallAndClose,
	OpenTextualInputThenCallAndClose,
	OpenTextualOutputThenCallAndClose,
	
	BytesReadCollect,
	BytesReadExtend,
	BytesReadCopy,
	
	StringReadCollect,
	StringReadExtend,
	
	ByteWrite,
	BytesWrite,
	
	CharacterWrite,
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive3 {
	
	BytesReadCopy,
	
	BytesWrite,
	
	StringWrite,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive4 {
	
	BytesReadCopy,
	
	BytesWrite,
	
	StringWrite,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitive5 {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveN {}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum PortPrimitiveV {
	
	/*
	ByteReady,
	BytePeek,
	ByteRead,
	
	CharacterReady,
	CharacterPeek,
	CharacterRead,
	
	BytesReadCollect,
	BytesReadExtend,
	BytesReadCopy,
	BytesReadChunk,
	BytesReadLine,
	
	StringReadCollect,
	StringReadExtend,
	StringReadChunk,
	StringReadLine,
	
	ValueRead,
	
	ByteWrite,
	BytesWrite,
	
	CharacterWrite,
	StringWrite,
	
	ValueWrite,
	ValueWriteShared,
	ValueWriteSimple,
	ValueDisplay,
	
	NewLine,
	FlushOutput,
	*/
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_0_evaluate (primitive : PortPrimitive0, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive0::OutputToBytes =>
			return port_bytes_writer_new (None),
		
		PortPrimitive0::OutputToString =>
			return port_string_writer_new (None),
		
		PortPrimitive0::CurrentInput =>
			return Port::new_stdin () .into_0 (),
		
		PortPrimitive0::CurrentOutput =>
			return Port::new_stdout () .into_0 (),
		
		PortPrimitive0::CurrentError =>
			return Port::new_stderr () .into_0 (),
		
		PortPrimitive0::Eof =>
			return PORT_EOF.into_0 (),
		
		PortPrimitive0::ByteReady =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytePeek =>
			fail_unimplemented! (0),
		
		PortPrimitive0::ByteRead =>
			fail_unimplemented! (0),
		
		PortPrimitive0::CharacterReady =>
			fail_unimplemented! (0),
		
		PortPrimitive0::CharacterPeek =>
			fail_unimplemented! (0),
		
		PortPrimitive0::CharacterRead =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytesReadCollect =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytesReadExtend =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytesReadCopy =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytesReadChunk =>
			fail_unimplemented! (0),
		
		PortPrimitive0::BytesReadLine =>
			fail_unimplemented! (0),
		
		PortPrimitive0::StringReadCollect =>
			fail_unimplemented! (0),
		
		PortPrimitive0::StringReadExtend =>
			fail_unimplemented! (0),
		
		PortPrimitive0::StringReadChunk =>
			fail_unimplemented! (0),
		
		PortPrimitive0::StringReadLine =>
			fail_unimplemented! (0),
		
		PortPrimitive0::ValueRead =>
			fail_unimplemented! (0),
		
		PortPrimitive0::NewLine =>
			return port_output_newline_0 (&mut try! (PortBackend::new_stdout ()), None, Some (true)) .into_0 (),
		
		PortPrimitive0::FlushOutput =>
			fail_unimplemented! (0),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_1_evaluate (primitive : PortPrimitive1, input_1 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive1::InputFromBytes =>
			return port_bytes_reader_new (input_1),
		
		PortPrimitive1::InputFromString =>
			return port_string_reader_new (input_1),
		
		PortPrimitive1::OutputToBytesFinalize =>
			return port_bytes_writer_finalize (input_1),
		
		PortPrimitive1::OutputToStringFinalize =>
			return port_string_writer_finalize (input_1),
		
		PortPrimitive1::OpenBinaryInput =>
			return port_file_reader_open (input_1, None),
		
		PortPrimitive1::OpenBinaryOutput =>
			return port_file_writer_open (input_1, None),
		
		PortPrimitive1::OpenTextualInput =>
			return port_file_reader_open (input_1, None),
		
		PortPrimitive1::OpenTextualOutput =>
			return port_file_writer_open (input_1, None),
		
		PortPrimitive1::IsInputOpen =>
			return is_port_input_open (input_1) .into_0 (),
		
		PortPrimitive1::IsOutputOpen =>
			return is_port_output_open (input_1) .into_0 (),
		
		PortPrimitive1::Close =>
			return port_close (input_1) .into_0 (),
		
		PortPrimitive1::CloseInput =>
			return port_input_close (input_1) .into_0 (),
		
		PortPrimitive1::CloseOutput =>
			return port_output_close (input_1) .into_0 (),
		
		PortPrimitive1::ByteReady =>
			return port_input_byte_ready (input_1) .into_0 (),
		
		PortPrimitive1::BytePeek =>
			return port_input_byte_peek (input_1),
		
		PortPrimitive1::ByteRead =>
			return port_input_byte_read (input_1),
		
		PortPrimitive1::CharacterReady =>
			return port_input_character_ready (input_1) .into_0 (),
		
		PortPrimitive1::CharacterPeek =>
			return port_input_character_peek (input_1),
		
		PortPrimitive1::CharacterRead =>
			return port_input_character_read (input_1),
		
		PortPrimitive1::BytesReadCollect =>
			return port_input_bytes_read_collect (input_1, None, None),
		
		PortPrimitive1::BytesReadExtend =>
			fail_unimplemented! (0),
		
		PortPrimitive1::BytesReadCopy =>
			fail_unimplemented! (0),
		
		PortPrimitive1::BytesReadChunk =>
			fail_unimplemented! (0),
		
		PortPrimitive1::BytesReadLine =>
			fail_unimplemented! (0),
		
		PortPrimitive1::StringReadCollect =>
			return port_input_string_read_collect (input_1, None, None),
		
		PortPrimitive1::StringReadExtend =>
			fail_unimplemented! (0),
		
		PortPrimitive1::StringReadChunk =>
			return port_input_read_chunk (input_1, None, None),
		
		PortPrimitive1::StringReadLine =>
			return port_input_read_line (input_1, Some (false), None, None),
		
		PortPrimitive1::ValueRead =>
			fail_unimplemented! (0xae3d8a9f), // deferred
		
		PortPrimitive1::ByteWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive1::BytesWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive1::CharacterWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive1::StringWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive1::ValueWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive1::ValueWriteShared =>
			fail_unimplemented! (0),
		
		PortPrimitive1::ValueWriteSimple =>
			fail_unimplemented! (0),
		
		PortPrimitive1::ValueDisplay =>
			return port_output_value_display_0 (&mut try! (PortBackend::new_stdout ()), input_1, None, None, Some (true)) .into_0 (),
		
		PortPrimitive1::NewLine =>
			fail_unimplemented! (0),
		
		PortPrimitive1::FlushOutput =>
			return port_output_flush (input_1) .into_0 (),
		
		PortPrimitive1::FileExists =>
			return port_file_exists (input_1) .into_0 (),
		
		PortPrimitive1::FileDelete =>
			return port_file_delete (input_1) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_2_evaluate (primitive : PortPrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive2::CallAndClose =>
			return port_call_and_close (input_1, input_2, evaluator),
		
		PortPrimitive2::OpenBinaryInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryInput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenBinaryOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenBinaryOutput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualInputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualInput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::OpenTextualOutputThenCallAndClose => {
			let port = try! (port_primitive_1_evaluate (PortPrimitive1::OpenTextualOutput, input_1, evaluator));
			return port_call_and_close (&port, input_2, evaluator);
		},
		
		PortPrimitive2::BytesReadCollect =>
			return port_input_bytes_read_collect (input_2, Some (input_1), None),
		
		PortPrimitive2::BytesReadExtend =>
			return port_input_bytes_read_extend (input_2, input_1, None, None),
		
		PortPrimitive2::BytesReadCopy =>
			return port_input_bytes_read_copy_range (input_2, input_1, None, None, None),
		
		PortPrimitive2::StringReadCollect =>
			return port_input_string_read_collect (input_2, Some (input_1), None),
		
		PortPrimitive2::StringReadExtend =>
			return port_input_string_read_extend (input_2, input_1, None, None),
		
		PortPrimitive2::ByteWrite =>
			return port_output_byte_write (input_2, input_1) .into_0 (),
		
		PortPrimitive2::BytesWrite =>
			return port_output_bytes_write (input_2, input_1) .into_0 (),
		
		PortPrimitive2::CharacterWrite =>
			return port_output_character_write (input_2, input_1) .into_0 (),
		
		PortPrimitive2::StringWrite =>
			return port_output_string_write (input_2, input_1) .into_0 (),
		
		PortPrimitive2::ValueWrite =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (input_2, input_1, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueWriteShared =>
			// TODO:  Add support for cyclic objects!
			return port_output_value_write (input_2, input_1, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueWriteSimple =>
			return port_output_value_write (input_2, input_1, None, None, None) .into_0 (),
		
		PortPrimitive2::ValueDisplay =>
			fail_unimplemented! (0),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_3_evaluate (primitive : PortPrimitive3, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive3::BytesReadCopy =>
			fail_unimplemented! (0),
		
		PortPrimitive3::BytesWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive3::StringWrite =>
			fail_unimplemented! (0),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_4_evaluate (primitive : PortPrimitive4, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		PortPrimitive4::BytesReadCopy =>
			fail_unimplemented! (0),
		
		PortPrimitive4::BytesWrite =>
			fail_unimplemented! (0),
		
		PortPrimitive4::StringWrite =>
			fail_unimplemented! (0),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_5_evaluate (primitive : PortPrimitive5, _input_1 : &Value, _input_2 : &Value, _input_3 : &Value, _input_4 : &Value, _input_5 : &Value, _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_evaluate (primitive : PortPrimitiveN, _inputs : &[&Value], _evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_0 (primitive : PortPrimitiveV) -> (Option<PortPrimitive0>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_1 (primitive : PortPrimitiveV) -> (Option<PortPrimitive1>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_2 (primitive : PortPrimitiveV) -> (Option<PortPrimitive2>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_3 (primitive : PortPrimitiveV) -> (Option<PortPrimitive3>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_4 (primitive : PortPrimitiveV) -> (Option<PortPrimitive4>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_5 (primitive : PortPrimitiveV) -> (Option<PortPrimitive5>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_v_alternative_n (primitive : PortPrimitiveV) -> (Option<PortPrimitiveN>) {
	match primitive {}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_0_attributes (_primitive : PortPrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_1_attributes (_primitive : PortPrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_2_attributes (_primitive : PortPrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_3_attributes (_primitive : PortPrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_4_attributes (_primitive : PortPrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_5_attributes (_primitive : PortPrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn port_primitive_n_attributes (_primitive : PortPrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

