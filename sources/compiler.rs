

use super::constants::exports::*;
use super::contexts::exports::*;
use super::errors::exports::*;
use super::expressions::exports::*;
use super::operators::exports::*;
use super::primitives::exports::*;
use super::procedures::exports::*;
use super::runtime::exports::*;
use super::values::exports::*;




pub mod exports {
	pub use super::compile;
	pub use super::Compiler;
	pub use super::{CompilerContext, CompilerBindings, CompilerBinding};
}




pub fn compile (context : &Context, token : &Value) -> (Outcome<Expression>) {
	return Compiler::new () .compile (context, token);
}




pub struct Compiler {}


impl Compiler {
	
	
	
	
	pub fn new () -> (Compiler) {
		return Compiler {};
	}
	
	pub fn compile (&self, context : &Context, token : &Value) -> (Outcome<Expression>) {
		let compilation = CompilerContext::new (CompilerBindings::Globals1 (context.clone ()));
		let (_compilation, expression) = try! (self.compile_0 (compilation, token.clone ()));
		succeed! (expression);
	}
	
	
	
	
	pub fn compile_0 (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		match token.class () {
			
			ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::String =>
				return self.compile_syntax_quote (compilation, token),
			
			ValueClass::Symbol =>
				return self.compile_symbol (compilation, token.into ()),
			
			ValueClass::Pair =>
				return self.compile_form (compilation, token.into ()),
			
			ValueClass::Bytes =>
				fail_unimplemented! (0xe7db25d8),
			ValueClass::Array =>
				fail_unimplemented! (0xe7db25d8),
			
			ValueClass::Error =>
				fail_panic! (0x2aa7bc60),
			
			ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
				fail_panic! (0xaf6f1288),
			
			ValueClass::Binding | ValueClass::Context =>
				fail_panic! (0x5f0d7003),
			
			ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
				fail_panic! (0x841d4d00),
			
		}
	}
	
	
	
	
	pub fn compile_vec_0 (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, ExpressionVec)>) {
		let mut expressions = ExpressionVec::with_capacity (tokens.len ());
		let mut compilation = compilation;
		for token in tokens.into_iter () {
			let (compilation_1, expression) = try! (self.compile_0 (compilation, token));
			compilation = compilation_1;
			expressions.push (expression);
		}
		succeed! ((compilation, expressions));
	}
	
	
	
	
	pub fn compile_symbol (&self, compilation : CompilerContext, identifier : Symbol) -> (Outcome<(CompilerContext, Expression)>) {
		let mut compilation = compilation;
		match try! (compilation.bindings.resolve (identifier)) {
			CompilerBinding::Undefined =>
				fail! (0xc6825cfd),
			CompilerBinding::Binding (binding) =>
				succeed! ((compilation, Expression::BindingGet (binding))),
			CompilerBinding::Register (index) =>
				succeed! ((compilation, Expression::RegisterGet (index))),
		}
	}
	
	
	
	
	pub fn compile_form (&self, compilation : CompilerContext, form : Pair) -> (Outcome<(CompilerContext, Expression)>) {
		
		match try! (self.compile_form_0 (compilation, form.clone ())) {
			
			(compilation, Some ((syntax, tokens))) =>
				return self.compile_syntax_call (compilation, syntax, tokens),
			
			(compilation, None) =>
				return self.compile_procedure_call (compilation, form.left () .clone (), form.right () .clone ()),
		}
	}
	
	
	pub fn compile_form_0 (&self, compilation : CompilerContext, token : Pair) -> (Outcome<(CompilerContext, Option<(SyntaxPrimitive, Value)>)>) {
		
		let mut compilation = compilation;
		let callable = token.left () .clone ();
		let arguments = token.right () .clone ();
		
		match callable.class () {
			
			ValueClass::Symbol => {
				if let Some (callable) = try! (compilation.bindings.resolve_value (callable.into ())) {
					match callable.class () {
						ValueClass::SyntaxPrimitive =>
							succeed! ((compilation, Some ((callable.into (), arguments)))),
						_ =>
							succeed! ((compilation, None)),
					}
				} else {
					succeed! ((compilation, None));
				}
			},
			
			ValueClass::SyntaxPrimitive =>
				succeed! ((compilation, Some ((callable.into (), arguments)))),
			
			_ =>
				succeed! ((compilation, None)),
			
		}
	}
	
	
	
	
	pub fn compile_procedure_call (&self, compilation : CompilerContext, procedure : Value, arguments : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, procedure) = try! (self.compile_0 (compilation, procedure));
		let (compilation, arguments) = try! (self.compile_vec_0 (compilation, try! (vec_clone_list (&arguments))));
		
		let expression = Expression::ProcedureCall (procedure.into (), arguments);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	pub fn compile_syntax_call (&self, compilation : CompilerContext, syntax : SyntaxPrimitive, tokens : Value) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens = try! (vec_clone_list (&tokens));
		let tokens_count = tokens.len ();
		
		match syntax {
			
			SyntaxPrimitive::Primitive1 (syntax) =>
				if tokens_count == 1 {
					let tokens = vec_explode_1! (tokens);
					match syntax {
						
						SyntaxPrimitive1::Quote =>
							return self.compile_syntax_quote (compilation, tokens),
						
						SyntaxPrimitive1::QuasiQuote =>
							return self.compile_syntax_quasi_quote (compilation, tokens),
						
						SyntaxPrimitive1::UnQuote | SyntaxPrimitive1::UnQuoteSplicing =>
							fail! (0x99b4857b),
						
					}
				} else {
					fail! (0x421da1f1);
				},
			
			SyntaxPrimitive::Primitive2 (syntax) =>
				if tokens_count == 2 {
					match syntax {
						
						SyntaxPrimitive2::Define =>
							return self.compile_syntax_define (compilation, tokens),
						
						SyntaxPrimitive2::DefineValues =>
							fail_unimplemented! (0x2f87acf0),
						
						SyntaxPrimitive2::Set =>
							fail_unimplemented! (0x19d7ebad),
						
						SyntaxPrimitive2::SetValues =>
							fail_unimplemented! (0xeea43320),
						
					}
				} else {
					fail! (0x9d9b6a94);
				},
			
			SyntaxPrimitive::Primitive3 (syntax) =>
				if tokens_count == 3 {
					match syntax {
						
						SyntaxPrimitive3::If =>
							return self.compile_syntax_if (compilation, tokens),
						
					}
				} else {
					fail! (0xd76f0ad2);
				},
			
			SyntaxPrimitive::PrimitiveN (syntax) =>
				match syntax {
					
					SyntaxPrimitiveN::And | SyntaxPrimitiveN::Or =>
						return self.compile_syntax_and_or (compilation, syntax, tokens),
					
					SyntaxPrimitiveN::Begin =>
						return self.compile_syntax_begin (compilation, tokens),
					
					SyntaxPrimitiveN::When | SyntaxPrimitiveN::Unless =>
						return self.compile_syntax_when_unless (compilation, syntax, tokens),
					
					SyntaxPrimitiveN::Cond =>
						return self.compile_syntax_cond (compilation, tokens),
					
					SyntaxPrimitiveN::Case =>
						return self.compile_syntax_case (compilation, tokens),
					
					SyntaxPrimitiveN::Do =>
						return self.compile_syntax_do (compilation, tokens),
					
					SyntaxPrimitiveN::Locals =>
						return self.compile_syntax_locals (compilation, tokens),
					
					SyntaxPrimitiveN::Let =>
						return self.compile_syntax_let (compilation, tokens),
					
					SyntaxPrimitiveN::LetValues =>
						fail_unimplemented! (0x5f0e99b2),
					
					SyntaxPrimitiveN::Define =>
						return self.compile_syntax_define (compilation, tokens),
					
					SyntaxPrimitiveN::Lambda =>
						return self.compile_syntax_lambda (compilation, None, tokens),
					
				},
			
			SyntaxPrimitive::Auxiliary =>
				fail! (0x1aed14f3),
			
			SyntaxPrimitive::Reserved =>
				fail! (0x1aed14f3),
			
			SyntaxPrimitive::Unimplemented =>
				fail_unimplemented! (0xa4e41f62),
			
		}
	}
	
	
	
	
	pub fn compile_syntax_and_or (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveN, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		
		let expression = Expression::SyntaxPrimitiveCall (syntax.into (), statements);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	pub fn compile_syntax_begin (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		
		let expression = Expression::Sequence (statements);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	pub fn compile_syntax_if (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		let (guard, if_true, if_false) = vec_explode_3! (statements);
		
		let conditions = vec! [
				(false, guard, if_true),
				(false, TRUE.into (), if_false),
			];
		let expression = Expression::Conditional (conditions);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	pub fn compile_syntax_when_unless (&self, compilation : CompilerContext, syntax : SyntaxPrimitiveN, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let tokens_count = tokens.len ();
		if tokens_count < 2 {
			fail! (0x3c364a9f);
		}
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, tokens));
		let (guard, statements) = vec_explode_1n! (statements);
		let statements = Expression::Sequence (statements);
		
		let negated = match syntax {
			SyntaxPrimitiveN::When =>
				false,
			SyntaxPrimitiveN::Unless =>
				true,
			_ =>
				fail_panic! (0x500d298f),
		};
		
		let conditions = vec! [
				(negated, guard, statements),
			];
		let expression = Expression::Conditional (conditions);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	#[ allow ( unused_variables )]
	pub fn compile_syntax_cond (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0xccfe6c4e);
	}
	
	
	
	
	#[ allow ( unused_variables )]
	pub fn compile_syntax_case (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0x2b2c2718);
	}
	
	
	
	
	#[ allow ( unused_variables )]
	pub fn compile_syntax_do (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0x4ff860c1);
	}
	
	
	
	
	pub fn compile_syntax_locals (&self, compilation : CompilerContext, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let compilation = try! (compilation.fork_locals (false));
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		let (compilation, registers) = try! (compilation.unfork_locals ());
		
		let statements = Expression::Sequence (statements);
		
		let registers_count = registers.len ();
		if registers_count == 0 {
			succeed! ((compilation, statements));
		} else {
			let expression = Expression::RegisterClosure (statements.into (), registers);
			succeed! ((compilation, expression));
		}
	}
	
	
	
	
	#[ allow ( unused_variables )]
	pub fn compile_syntax_let (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		fail_unimplemented! (0x9cd48665);
	}
	
	
	
	
	pub fn compile_syntax_define (&self, compilation : CompilerContext, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x4481879c);
		}
		let (signature, statements) = vec_explode_1n! (tokens);
		let mut compilation = compilation;
		
		let (compilation, binding, expression) = match signature.class () {
			
			ValueClass::Symbol => {
				
				if statements.len () != 1 {
					fail! (0xc364edf8);
				}
				
				let identifier = try_into_symbol! (signature);
				let statement = vec_explode_1! (statements);
				
				let binding = try! (compilation.bindings.define (identifier));
				let (compilation, expression) = try! (self.compile_0 (compilation, statement));
				
				(compilation, binding, expression)
			},
			
			ValueClass::Pair => {
				
				if statements.len () < 1 {
					fail! (0x48c70de5);
				}
				
				let (signature, argument_rest) = try! (vec_clone_list_dotted (&signature));
				let (identifier, arguments_positional) = vec_explode_1n! (signature);
				
				let identifier = try_into_symbol! (identifier);
				let arguments_positional = try_vec_map! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				
				let binding = try! (compilation.bindings.define (identifier.clone ()));
				let (compilation, expression) = try! (self.compile_syntax_lambda_0 (compilation, Some (identifier), arguments_positional, argument_rest, statements));
				
				(compilation, binding, expression)
			},
			
			_ =>
				fail! (0x0f0edc26),
		};
		
		match binding {
			
			CompilerBinding::Undefined =>
				fail! (0x42370d15),
			
			CompilerBinding::Binding (binding) => {
				let expression = Expression::BindingInitialize (binding, expression.into ());
				succeed! ((compilation, expression));
			},
			
			CompilerBinding::Register (index) => {
				let expression = Expression::RegisterInitialize (index, expression.into ());
				succeed! ((compilation, expression));
			},
			
		}
	}
	
	
	
	
	pub fn compile_syntax_lambda (&self, compilation : CompilerContext, identifier : Option<Symbol>, tokens : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		if tokens.len () < 2 {
			fail! (0x2dfd91d1);
		}
		
		let (arguments, statements) = vec_explode_1n! (tokens);
		let (arguments_positional, argument_rest) = match arguments.class () {
			ValueClass::Symbol =>
				(StdVec::new (), Some (Symbol::from (arguments))),
			ValueClass::Pair | ValueClass::Null => {
				let (arguments_positional, argument_rest) = try! (vec_clone_list_dotted (&arguments));
				let arguments_positional = try_vec_map! (arguments_positional, value, Symbol::try_from (value));
				let argument_rest = try_option_map! (argument_rest, Symbol::try_from (argument_rest));
				(arguments_positional, argument_rest)
			},
			_ =>
				fail! (0x70773cab),
		};
		
		return self.compile_syntax_lambda_0 (compilation, identifier, arguments_positional, argument_rest, statements);
	}
	
	
	pub fn compile_syntax_lambda_0 (&self, compilation : CompilerContext, identifier : Option<Symbol>, arguments_positional : StdVec<Symbol>, argument_rest : Option<Symbol>, statements : ValueVec) -> (Outcome<(CompilerContext, Expression)>) {
		
		let arguments_count = arguments_positional.len () + if argument_rest.is_some () { 1 } else { 0 };
		
		let compilation = try! (compilation.fork_locals (true));
		
		let mut compilation = try! (compilation.fork_locals (true));
		for identifier in &arguments_positional {
			try! (compilation.bindings.define (identifier.clone ()));
		}
		if let Some (ref identifier) = argument_rest {
			try! (compilation.bindings.define (identifier.clone ()));
		}
		
		let (compilation, statements) = try! (self.compile_vec_0 (compilation, statements));
		
		let (compilation, mut registers_local) = try! (compilation.unfork_locals ());
		let (compilation, registers_closure) = try! (compilation.unfork_locals ());
		
		let registers_local = registers_local.split_off (arguments_count);
		
		let statements = Expression::Sequence (statements);
		
		let template = LambdaTemplate {
				identifier : identifier,
				arguments_positional : arguments_positional,
				argument_rest : argument_rest,
			};
		
		let expression = Expression::Lambda (StdBox::new (template), statements.into (), registers_closure, registers_local);
		
		succeed! ((compilation, expression));
	}
	
	
	
	
	pub fn compile_syntax_quote (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		succeed! ((compilation, Expression::Value (token)));
	}
	
	
	
	
	pub fn compile_syntax_quasi_quote (&self, compilation : CompilerContext, token : Value) -> (Outcome<(CompilerContext, Expression)>) {
		return self.compile_syntax_quasi_quote_0 (compilation, token, true, false);
	}
	
	
	pub fn compile_syntax_quasi_quote_0 (&self, compilation : CompilerContext, token : Value, top : bool, spliceable : bool) -> (Outcome<(CompilerContext, Expression)>) {
		
		fn splice <ExpressionInto : StdInto<Expression>> (expression : ExpressionInto, spliceable : bool) -> (Expression) {
			let expression = expression.into ();
			if spliceable {
				Expression::ProcedureCall (ListPrimitive1::List.into (), vec! [expression])
			} else {
				expression
			}
		}
		
		match token.class () {
			
			ValueClass::Null | ValueClass::Void | ValueClass::Undefined =>
				succeed! ((compilation, splice (token, spliceable))),
			ValueClass::Boolean | ValueClass::NumberInteger | ValueClass::NumberReal | ValueClass::Character =>
				succeed! ((compilation, splice (token, spliceable))),
			ValueClass::String | ValueClass::Bytes =>
				succeed! ((compilation, splice (token, spliceable))),
			
			ValueClass::Symbol =>
				succeed! ((compilation, splice (token, spliceable))),
			ValueClass::Array =>
				fail_unimplemented! (0x0d99c57b),
			
			ValueClass::Error =>
				fail! (0x9681733a),
			ValueClass::Lambda | ValueClass::ProcedurePrimitive | ValueClass::SyntaxPrimitive =>
				fail! (0x251a7fd0),
			
			ValueClass::Binding | ValueClass::Context =>
				fail! (0xfa7ef6f6),
			
			ValueClass::Number | ValueClass::List | ValueClass::ListProper | ValueClass::ListDotted | ValueClass::True | ValueClass::False | ValueClass::Procedure | ValueClass::Syntax =>
				fail! (0x841d4d00),
			
			ValueClass::Pair => {
				
				let compilation = match try! (self.compile_form_0 (compilation, token.clone () .into ())) {
					
					(compilation, Some ((syntax, tokens))) => {
						let tokens = try! (vec_clone_list (&tokens));
						let tokens_count = tokens.len ();
						match syntax {
							
							SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuote) =>
								if tokens_count == 1 {
									let token = vec_explode_1! (tokens);
									let (compilation, element) = try! (self.compile_0 (compilation, token));
									succeed! ((compilation, splice (element, spliceable)));
								} else {
									fail! (0x9dc44267);
								},
							
							SyntaxPrimitive::Primitive1 (SyntaxPrimitive1::UnQuoteSplicing) =>
								if tokens_count == 1 {
									if spliceable {
										let token = vec_explode_1! (tokens);
										let (compilation, element) = try! (self.compile_0 (compilation, token));
										succeed! ((compilation, element));
									} else {
										fail! (0x47356961);
									}
								} else {
									fail! (0xe0c45124);
								},
							
							_ =>
								compilation,
						}
					},
					
					(compilation, None) =>
						compilation,
					
				};
				
				let mut compilation = compilation;
				let mut elements = ExpressionVec::new ();
				let mut cursor = &token;
				loop {
					match cursor.class () {
						
						ValueClass::Pair => {
							let pair = cursor.as_ref () as &Pair;
							let (compilation_1, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, pair.left () .clone (), false, true));
							compilation = compilation_1;
							elements.push (element);
							cursor = pair.right ();
						},
						
						ValueClass::Null =>
							break,
						
						_ => {
							let (compilation_1, element) = try! (self.compile_syntax_quasi_quote_0 (compilation, cursor.clone (), false, true));
							compilation = compilation_1;
							elements.push (element);
							break;
						},
						
					}
				}
				
				let expression = Expression::ProcedureCall (ListPrimitiveN::Append.into (), elements);
				
				let expression = if top {
					expression
				} else {
					Expression::ProcedureCall (ListPrimitive2::Pair.into (), vec! [ expression, NULL.into () ])
				};
				
				succeed! ((compilation, expression));
			},
			
		}
	}
	
	
	
	
}




pub struct CompilerContext {
	bindings : CompilerBindings,
}


impl CompilerContext {
	
	pub fn new (bindings : CompilerBindings) -> (CompilerContext) {
		return CompilerContext {
				bindings : bindings,
			};
	}
	
	pub fn fork_locals (self, force : bool) -> (Outcome<CompilerContext>) {
		let bindings = try! (self.bindings.fork_locals (force));
		succeed! (CompilerContext::new (bindings));
	}
	
	pub fn unfork_locals (self) -> (Outcome<(CompilerContext, StdVec<RegistersBindingTemplate>)>) {
		let (bindings, registers) = try! (self.bindings.unfork_locals ());
		succeed! ((CompilerContext::new (bindings), registers));
	}
}




#[ allow (dead_code) ]
pub enum CompilerBindings {
	None,
	Globals1 (Context),
	Globals2 (StdBox<CompilerBindings>, Context),
	Locals (StdBox<CompilerBindings>, StdMap<Symbol, CompilerBinding>, StdVec<RegistersBindingTemplate>, usize),
}


#[ derive (Clone) ]
#[ allow (dead_code) ]
pub enum CompilerBinding {
	Undefined,
	Binding (Binding),
	Register (usize),
}


impl CompilerBindings {
	
	
	pub fn fork_locals (self, force : bool) -> (Outcome<CompilerBindings>) {
		if force {
			succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), 1));
		} else {
			match self {
				CompilerBindings::None =>
					fail! (0xad3e033b),
				CompilerBindings::Globals1 (_) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None))),
				CompilerBindings::Globals2 (_, _) =>
					succeed! (CompilerBindings::Globals2 (StdBox::new (self), Context::new (None))),
				CompilerBindings::Locals (_, _, _, depth) =>
					succeed! (CompilerBindings::Locals (StdBox::new (self), StdMap::new (), StdVec::new (), depth + 1)),
			}
		}
	}
	
	pub fn unfork_locals (self) -> (Outcome<(CompilerBindings, StdVec<RegistersBindingTemplate>)>) {
		match self {
			CompilerBindings::None =>
				fail! (0x98657e5a),
			CompilerBindings::Globals1 (_) =>
				fail! (0xdd470d36),
			CompilerBindings::Globals2 (parent, _) =>
				succeed! ((*parent, StdVec::new ())),
			CompilerBindings::Locals (parent, _, registers, _) =>
				succeed! ((*parent, registers)),
		}
	}
	
	
	pub fn resolve (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				succeed! (CompilerBinding::Undefined),
			CompilerBindings::Globals1 (ref context) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (binding));
				} else {
					succeed! (CompilerBinding::Undefined);
				},
			CompilerBindings::Globals2 (ref mut parent, ref context) =>
				if let Some (binding) = try! (context.resolve (&identifier)) {
					succeed! (CompilerBinding::Binding (binding));
				} else {
					return parent.resolve (identifier);
				},
			CompilerBindings::Locals (ref mut parent, ref mut cached, ref mut registers, _depth) => {
				if let Some (binding) = cached.get (&identifier) {
					succeed! (binding.clone ());
				} /*else*/ {
					match try! (parent.resolve (identifier.clone ())) {
						CompilerBinding::Undefined =>
							succeed! (CompilerBinding::Undefined),
						binding @ CompilerBinding::Binding (_) =>
							succeed! (binding),
						CompilerBinding::Register (parent_index) => {
							let self_index = registers.len ();
							let self_binding = CompilerBinding::Register (self_index);
							let template = RegistersBindingTemplate {
									identifier : Some (identifier.clone ()),
									borrow : Some (parent_index),
									value : None,
									immutable : false,
								};
							registers.push (template);
							cached.insert (identifier, self_binding.clone ());
							succeed! (self_binding);
						}
					}
				}
			},
		}
	}
	
	pub fn define (&mut self, identifier : Symbol) -> (Outcome<CompilerBinding>) {
		match *self {
			CompilerBindings::None =>
				fail! (0xd943456d),
			CompilerBindings::Globals1 (ref context) | CompilerBindings::Globals2 (_, ref context) => {
				let template = ContextBindingTemplate {
						identifier : identifier,
						value : None,
						immutable : false,
					};
				let binding = try! (context.define (&template));
				succeed! (CompilerBinding::Binding (binding));
			},
			CompilerBindings::Locals (ref _parent, ref mut cached, ref mut registers, _depth) => {
				let index = registers.len ();
				let binding = CompilerBinding::Register (index);
				let template = RegistersBindingTemplate {
						identifier : Some (identifier.clone ()),
						borrow : None,
						value : None,
						immutable : false,
					};
				registers.push (template);
				cached.insert (identifier, binding.clone ());
				succeed! (binding);
			},
		}
	}
	
	
	pub fn resolve_value (&mut self, identifier : Symbol) -> (Outcome<Option<Value>>) {
		match try! (self.resolve (identifier)) {
			CompilerBinding::Undefined =>
				succeed! (None),
			CompilerBinding::Binding (binding) =>
				succeed! (Some (try! (binding.get ()))),
			CompilerBinding::Register (_index) =>
				succeed! (None),
		}
	}
}
