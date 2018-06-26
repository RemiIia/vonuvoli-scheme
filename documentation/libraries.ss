
(library
	
	
	
	
	(identifier vonuvoli:r7rs)
	
	(title "R7RS functionality with Vonuvoli-Scheme extensions")
	
	
	
	
	(categories
		
		(r7rs)
		(r7rs:base (parent r7rs))
		(r7rs:case-lambda (parent r7rs))
		(r7rs:char (parent r7rs))
		(r7rs:complex (parent r7rs))
		(r7rs:cxr (parent r7rs))
		(r7rs:eval (parent r7rs))
		(r7rs:file (parent r7rs))
		(r7rs:inexact (parent r7rs))
		(r7rs:lazy (parent r7rs))
		(r7rs:load (parent r7rs))
		(r7rs:process-context (parent r7rs))
		(r7rs:r5rs (parent r7rs))
		(r7rs:read (parent r7rs))
		(r7rs:repl (parent r7rs))
		(r7rs:time (parent r7rs))
		(r7rs:write (parent r7rs))
		
		(r7rs-x (parent r7rs))
		(r7rs-x:types (parent r7rs-x))
		(r7rs-x:types-disjoint (parent r7rs-x:types))
		
		(vs)
		(vs:arithmetic (parent vs))
		(vs:associations (parent vs))
		(vs:bytes (parent vs))
		(vs:booleans (parent vs))
		(vs:conversions (parent vs))
		(vs:globals (parent vs))
		(vs:file-system (parent vs))
		(vs:characters (parent vs))
		(vs:comparisons (parent vs))
		(vs:compiler (parent vs))
		(vs:contexts (parent vs))
		(vs:continuations (parent vs))
		(vs:control (parent vs))
		(vs:equivalence (parent vs))
		(vs:errors (parent vs))
		(vs:evaluator (parent vs))
		(vs:functions (parent vs))
		(vs:lambda (parent vs))
		(vs:lists (parent vs))
		(vs:loops (parent vs))
		(vs:modules (parent vs))
		(vs:pairs (parent vs))
		(vs:parameters (parent vs))
		(vs:ports (parent vs))
		(vs:ports:input (parent vs:ports))
		(vs:ports:output (parent vs:ports))
		(vs:ports:open (parent vs:ports))
		(vs:ports:values (parent vs:ports))
		(vs:promises (parent vs))
		(vs:quotation (parent vs))
		(vs:records (parent vs))
		(vs:strings (parent vs))
		(vs:symbols (parent vs))
		(vs:syntaxes (parent vs))
		(vs:system (parent vs))
		(vs:types (parent vs))
		(vs:unimplemented (parent vs))
		(vs:unsupported (parent vs))
		(vs:values (parent vs))
		(vs:vectors (parent vs))
		
	)
	
	
	
	
	(definitions
		
		
		
		
		(define-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					((keyword identifier))
				(_ keyword @syntax-transformer)
			))
		
		(let-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(keyword identifier)
						(syntaxes pattern
							()
							((keyword @syntax-transformer) ...))
						(expression expression))
				(_ syntaxes)
				(_ syntaxes expression ...)
			))
		
		(letrec-syntax (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(keyword identifier)
						(syntaxes pattern
							()
							((keyword @syntax-transformer) ...))
						(expression expression))
				(_ syntaxes)
				(_ syntaxes expression ...)
			))
		
		
		
		
		(syntax-rules (category r7rs:base vs:syntaxes vs:unsupported) (type syntax))
		
		(syntax-error (category r7rs:base vs:syntaxes vs:unsupported) (type syntax)
			(syntax-rules
					(
						(message value string)
						(argument value any))
				(_ message)
				(_ message argument ...)
			))
		
		
		
		
		(_ (category r7rs:base vs:syntaxes) (type auxiliary-syntax))
		(... (category r7rs:base vs:syntaxes) (type auxiliary-syntax))
		(=> (category r7rs:base vs:syntaxes) (type auxiliary-syntax))
		(else (category r7rs:base vs:syntaxes) (type auxiliary-syntax))
		
		
		(quote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token)))
		
		(quasiquote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token)))
		
		(unquote (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token)))
		
		(unquote-splicing (category r7rs:base vs:syntaxes vs:quotation) (type syntax)
			(syntax-rules ((token value any)) (_ token)))
		
		
		
		
		(lambda (category r7rs:base vs:lambda) (type syntax)
			(syntax-rules
					(
						(argument identifier)
						(argument-rest identifier)
						(arguments pattern
							()
							(argument ...)
							(argument ... . argument-rest)
							argument-rest)
						(expression expression))
				(_ arguments expression ...)
			))
		
		(case-lambda (category r7rs:case-lambda vs:lambda) (type syntax)
			(syntax-rules
					(
						(argument identifier)
						(argument-rest identifier)
						(arguments pattern
							()
							(argument ...)
							(argument ... . argument-rest)
							argument-rest)
						(expression expression))
				(_
					(arguments expression)
					...)
			))
		
		
		
		
		(define (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(argument identifier)
						(argument-rest identifier)
						(expression expression))
				(_ variable expression)
				(_ (variable) expression ...)
				(_ (variable argument ...) expression ...)
				(_ (variable argument ... . argument-rest) expression ...)
				(_ (variable . argument-rest) expression ...)
			))
		
		
		(let (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(function identifier)
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
				(_ function bindings expression ...)
			))
		
		(let* (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
			))
		
		(letrec (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
			))
		
		(letrec* (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							(variable initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
			))
		
		
		(set! (category r7rs:base vs:contexts) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(expression expression))
				(_ variable expression)
			))
		
		
		
		
		(define-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(expression expression))
				(_ (variable ...) expression)
			))
		
		(let-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							((variable ...) initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
			))
		
		(let*-values (category r7rs:base vs:contexts vs:values) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(initializer identifier)
						(binding pattern
							((variable ...) initializer))
						(bindings pattern
							()
							(binding ...))
						(expression expression))
				(_ bindings)
				(_ bindings expression ...)
			))
		
		
		
		
		(define-record-type (category r7rs:base vs:contexts vs:records) (type syntax)
			(syntax-rules
					(
						(type-identifier identifier)
						(constructor-identifier identifier)
						(predicate-identifier identifier)
						(field-identifier identifier)
						(field-accessor-identifier identifier)
						(field-mutator-identifier identifier)
						(constructor-descriptor pattern
							constructor-identifier
							(constructor-identifier field-identifier ...))
						(field-descriptor pattern
							(field-identifier field-accessor-identifier)
							(field-identifier field-accessor-identifier field-mutator-identifier)))
				(_ type-identifier constructor-descriptor predicate-identifier field-descriptor ...)
			))
		
		
		
		
		(begin (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...)
			))
		
		(and (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...)
			))
		
		(or (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					((expression expression))
				(_)
				(_ expression ...)
			))
		
		
		(if (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression)
						(else-expression expression))
				(_ condition then-expression)
				(_ condition then-expression else-expression)
			))
		
		(unless (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression))
				(_ condition then-expression ...)
			))
		
		(when (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(condition expression)
						(then-expression expression))
				(_ condition then-expression ...)
			))
		
		
		(cond (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(else literal)
						(condition expression)
						(then-expression expression)
						(clause pattern
							(condition)
							(condition then-expression ...)
							(else)
							(else then-expression ...)))
				(_)
				(_ clause ...)
			))
		
		(case (category r7rs:base vs:control) (type syntax)
			(syntax-rules
					(
						(else literal)
						(value expression)
						(variant value any)
						(then-expression expression)
						(clause pattern
							((variant ...))
							((variant ...) then-expression ...)
							(else)
							(else then-expression ...)))
				(_ value)
				(_ value clause ...)
			))
		
		
		(do (category r7rs:base vs:control vs:loops) (type syntax)
			(syntax-rules
					(
						(binding-variable identifier)
						(binding-initializer expression)
						(binding-updater expression)
						(binding pattern
							(binding-variable binding-initializer)
							(binding-variable binding-initializer binding-updater))
						(bindings pattern
							()
							(binding ...))
						(exit-test expression)
						(exit-expression expression)
						(exit-clause pattern
							(exit-test)
							(exit-test exit-expression))
						(iteration-expression expression))
				(_ bindings exit-clause)
				(_ bindings exit-clause iteration-expression ...)
			))
		
		
		
		
		(eq? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean)))
		(eqv? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean)))
		(equal? (category r7rs:base vs:equivalence) (type comparator=)
			(signature ((any ...) -> boolean)))
		
		(not (category r7rs:base vs:equivalence) (type predicate)
			(signature (any -> boolean)))
		
		
		
		
		(boolean? (category r7rs:base vs:booleans vs:types) (type type-predicate))
		(boolean=? (category r7rs:base vs:booleans vs:comparisons vs:equivalence) (type comparator=))
		
		
		(symbol? (category r7rs:base vs:symbols vs:types) (type type-predicate))
		(symbol=? (category r7rs:base vs:symbols vs:comparisons vs:equivalence) (type comparator=))
		
		
		
		
		(number? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(real? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(rational? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(complex? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		
		(exact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(inexact? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		(exact-integer? (category r7rs:base vs:arithmetic vs:types) (type type-predicate))
		
		(zero? (category r7rs:base vs:arithmetic) (type predicate))
		(positive? (category r7rs:base vs:arithmetic) (type predicate))
		(odd? (category r7rs:base vs:arithmetic) (type predicate))
		(even? (category r7rs:base vs:arithmetic) (type predicate))
		
		(= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator=))
		(< (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<))
		(> (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>))
		(<= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator<=))
		(>= (category r7rs:base vs:arithmetic vs:comparisons) (type comparator>=))
		
		(+ (category r7rs:base vs:arithmetic) (type procedure))
		(- (category r7rs:base vs:arithmetic) (type procedure))
		(* (category r7rs:base vs:arithmetic) (type procedure))
		(/ (category r7rs:base vs:arithmetic) (type procedure))
		
		(abs (category r7rs:base vs:arithmetic) (type procedure))
		
		(floor/ (category r7rs:base vs:arithmetic) (type procedure))
		(floor-quotient (category r7rs:base vs:arithmetic) (type procedure))
		(floor-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias modulo))
		(truncate/ (category r7rs:base vs:arithmetic) (type procedure))
		(truncate-quotient (category r7rs:base vs:arithmetic) (type procedure) (alias quotient))
		(truncate-remainder (category r7rs:base vs:arithmetic) (type procedure) (alias remainder))
		
		(floor (category r7rs:base vs:arithmetic) (type procedure))
		(ceiling (category r7rs:base vs:arithmetic) (type procedure))
		(truncate (category r7rs:base vs:arithmetic) (type procedure))
		(round (category r7rs:base vs:arithmetic) (type procedure))
		
		(min (category r7rs:base vs:arithmetic) (type procedure))
		(max (category r7rs:base vs:arithmetic) (type procedure))
		(gcd (category r7rs:base vs:arithmetic) (type procedure))
		(lcm (category r7rs:base vs:arithmetic) (type procedure))
		
		(expt (category r7rs:base vs:arithmetic) (type procedure))
		(square (category r7rs:base vs:arithmetic) (type procedure))
		(exact-integer-sqrt (category r7rs:base vs:arithmetic) (type procedure))
		
		(rationalize (category r7rs:base vs:arithmetic vs:unsupported) (type procedure))
		(numerator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure))
		(denominator (category r7rs:base vs:arithmetic vs:unsupported) (type procedure))
		
		(inexact (category r7rs:complex vs:arithmetic) (type converter))
		(exact (category r7rs:complex vs:arithmetic) (type converter))
		
		(make-rectangular (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		(real-part (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		(imag-part (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		(make-polar (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		(magnitude (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		(angle (category r7rs:complex vs:arithmetic vs:unsupported) (type procedure))
		
		(sqrt (category r7rs:inexact vs:arithmetic) (type procedure))
		(exp (category r7rs:inexact vs:arithmetic) (type procedure))
		(log (category r7rs:inexact vs:arithmetic) (type procedure))
		
		(sin (category r7rs:inexact vs:arithmetic) (type procedure))
		(cos (category r7rs:inexact vs:arithmetic) (type procedure))
		(tan (category r7rs:inexact vs:arithmetic) (type procedure))
		(asin (category r7rs:inexact vs:arithmetic) (type procedure))
		(acos (category r7rs:inexact vs:arithmetic) (type procedure))
		(atan (category r7rs:inexact vs:arithmetic) (type procedure))
		
		(finite? (category r7rs:inexact vs:arithmetic) (type predicate))
		(infinite? (category r7rs:inexact vs:arithmetic) (type predicate))
		(nan? (category r7rs:inexact vs:arithmetic) (type predicate))
		
		
		
		
		(pair? (category r7rs:base vs:pairs vs:lists vs:types) (type type-predicate))
		(cons (category r7rs:base vs:pairs vs:lists) (type constructor))
		(car (category r7rs:base vs:pairs vs:lists) (type accessor))
		(cdr (category r7rs:base vs:pairs vs:lists) (type accessor))
		(set-car! (category r7rs:base vs:pairs vs:lists) (type mutator!))
		(set-cdr! (category r7rs:base vs:pairs vs:lists) (type mutator!))
		
		(caar (category r7rs:base vs:pairs vs:lists) (type accessor))
		(cadr (category r7rs:base vs:pairs vs:lists) (type accessor))
		
		(cdar (category r7rs:base vs:pairs vs:lists) (type accessor))
		(cddr (category r7rs:base vs:pairs vs:lists) (type accessor))
		
		(caaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cadar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		
		(cdaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cddar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		
		(caaaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caaadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caadar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caaddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cadaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cadadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(caddar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cadddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		
		(cdaaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdaadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdadar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdaddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cddaar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cddadr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cdddar (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		(cddddr (category r7rs:cxr vs:pairs vs:lists) (type accessor))
		
		
		
		(null? (category r7rs:base vs:lists vs:types) (type type-predicate))
		(list? (category r7rs:base vs:lists vs:types) (type type-predicate))
		
		(list (category r7rs:base vs:lists) (type constructor))
		(make-list (category r7rs:base vs:lists) (type constructor))
		
		(length (category r7rs:base vs:lists) (type procedure))
		(append (category r7rs:base vs:lists) (type procedure))
		(list-copy (category r7rs:base vs:lists) (type procedure))
		(reverse (category r7rs:base vs:lists) (type procedure))
		
		(list-ref (category r7rs:base vs:lists) (type accessor))
		(list-tail (category r7rs:base vs:lists) (type accessor))
		(list-set! (category r7rs:base vs:lists) (type mutator!))
		
		(member (category r7rs:base vs:lists) (type procedure))
		(memq (category r7rs:base vs:lists) (type procedure))
		(memv (category r7rs:base vs:lists) (type procedure))
		
		(assoc (category r7rs:base vs:lists vs:associations) (type procedure))
		(assqc (category r7rs:base vs:lists vs:associations) (type procedure))
		(assvc (category r7rs:base vs:lists vs:associations) (type procedure))
		
		(map (category r7rs:base vs:lists vs:functions vs:conversions vs:loops) (type map))
		(for-each (category r7rs:base vs:lists vs:functions vs:loops) (type for-each))
		
		
		
		
		(vector? (category r7rs:base vs:vectors vs:types) (type type-predicate))
		
		(vector (category r7rs:base vs:vectors) (type constructor))
		(make-vector (category r7rs:base vs:vectors) (type constructor))
		
		(vector-length (category r7rs:base vs:vectors) (type procedure))
		(vector-append (category r7rs:base vs:vectors) (type procedure))
		(vector-copy (category r7rs:base vs:vectors) (type accessor))
		(vector-copy! (category r7rs:base vs:vectors) (type mutator!))
		(vector-fill! (category r7rs:base vs:vectors) (type mutator!))
		
		(vector-ref (category r7rs:base vs:vectors) (type accessor))
		(vector-set! (category r7rs:base vs:vectors) (type mutator!))
		
		(vector->list (category r7rs:base vs:vectors vs:lists vs:conversions) (type converter))
		(list->vector (category r7rs:base vs:vectors vs:lists vs:conversions) (type converter))
		
		(vector-map (category r7rs:base vs:vectors vs:functions vs:conversions vs:loops) (type map))
		(vector-for-each (category r7rs:base vs:vectors vs:functions vs:loops) (type for-each))
		
		
		
		
		(string? (category r7rs:base vs:strings vs:types) (type type-predicate))
		
		(string (category r7rs:base vs:strings) (type constructor))
		(make-string (category r7rs:base vs:strings) (type constructor))
		
		(string-length (category r7rs:base vs:strings) (type procedure))
		(string-append (category r7rs:base vs:strings) (type constructor))
		(string-copy (category r7rs:base vs:strings) (type accessor))
		(string-copy! (category r7rs:base vs:strings) (type mutator!))
		(string-fill! (category r7rs:base vs:strings) (type mutator!))
		(substring  (category r7rs:base vs:strings) (type accessor))
		
		(string-ref (category r7rs:base vs:strings) (type accessor))
		(string-set! (category r7rs:base vs:strings) (type mutator!))
		
		(string=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator=))
		(string<? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator<))
		(string>? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator>))
		(string<=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator<=))
		(string>=? (category r7rs:base vs:strings vs:comparisons vs:equivalence) (type comparator>=))
		
		(string-ci=? (category r7rs:char vs:strings vs:comparisons vs:equivalence) (type comparator=))
		(string-ci<? (category r7rs:char vs:strings vs:comparisons) (type comparator<))
		(string-ci>? (category r7rs:char vs:strings vs:comparisons) (type comparator>))
		(string-ci<=? (category r7rs:char vs:strings vs:comparisons) (type comparator<=))
		(string-ci>=? (category r7rs:char vs:strings vs:comparisons) (type comparator>=))
		
		(number->string (category r7rs:base vs:strings vs:conversions) (type converter))
		(string->number (category r7rs:base vs:strings vs:conversions) (type converter))
		
		(symbol->string (category r7rs:base vs:strings vs:symbols vs:conversions) (type converter))
		(string->symbol (category r7rs:base vs:strings vs:symbols vs:conversions) (type converter))
		
		(list->string (category r7rs:base vs:strings vs:lists vs:conversions) (type converter))
		(string->list (category r7rs:base vs:strings vs:lists vs:conversions) (type converter))
		
		(vector->string (category r7rs:base vs:strings vs:vectors vs:conversions) (type converter))
		(string->vector (category r7rs:base vs:strings vs:vectors vs:conversions) (type converter))
		
		(string-map (category r7rs:base vs:strings vs:functions vs:conversions vs:loops) (type map))
		(string-for-each (category r7rs:base vs:strings vs:functions vs:loops) (type for-each))
		
		(string-upcase (category r7rs:char vs:strings vs:conversions) (type procedure))
		(string-downcase (category r7rs:char vs:strings vs:conversions) (type procedure))
		(string-foldcase (category r7rs:char vs:strings vs:conversions) (type procedure))
		
		
		
		
		(bytevector? (category r7rs:base vs:bytes) (type type-predicate))
		
		(bytevector (category r7rs:base vs:bytes) (type constructor))
		(make-bytevector (category r7rs:base vs:bytes) (type constructor))
		
		(bytevector-length (category r7rs:base vs:bytes) (type procedure))
		(bytevector-append (category r7rs:base vs:bytes) (type procedure))
		(bytevector-copy (category r7rs:base vs:bytes) (type procedure))
		(bytevector-copy! (category r7rs:base vs:bytes) (type procedure!))
		
		(bytevector-u8-ref (category r7rs:base vs:bytes) (type accessor))
		(bytevector-u8-set! (category r7rs:base vs:bytes) (type mutator!))
		
		(utf8->string (category r7rs:base vs:bytes vs:strings) (type converter))
		(string->utf8 (category r7rs:base vs:bytes vs:strings) (type converter))
		
		
		
		
		(port? (category r7rs:base vs:ports vs:types) (type type-predicate))
		
		(binary-port? (category r7rs:base vs:ports) (type predicate))
		(textual-port? (category r7rs:base vs:ports) (type predicate))
		
		(input-port? (category r7rs:base vs:ports:input) (type predicate))
		(input-port-open? (category r7rs:base vs:ports:input vs:ports:open) (type predicate))
		
		(output-port? (category r7rs:base vs:ports:output) (type predicate))
		(output-port-open? (category r7rs:base vs:ports:output vs:ports:open) (type predicate))
		
		
		(open-input-bytevector (category r7rs:base vs:ports:input vs:ports:open vs:bytes) (type procedure))
		(open-output-bytevector (category r7rs:base vs:ports:output vs:ports:open vs:bytes) (type procedure))
		(get-output-bytevector (category r7rs:base vs:ports:output vs:bytes) (type procedure))
		
		(open-input-string (category r7rs:base vs:ports:input vs:ports:open vs:strings) (type procedure))
		(open-output-string (category r7rs:base vs:ports:output vs:ports:open vs:strings) (type procedure))
		(get-output-string (category r7rs:base vs:ports:output vs:strings) (type procedure))
		
		
		(close-port (category r7rs:base vs:ports) (type procedure))
		(close-input-port (category r7rs:base vs:ports:input) (type procedure))
		(close-output-port (category r7rs:base vs:ports:output) (type procedure))
		
		
		(u8-ready? (category r7rs:base vs:ports:input vs:bytes) (type predicate))
		(peek-u8 (category r7rs:base vs:ports:input vs:bytes) (type procedure))
		(read-u8 (category r7rs:base vs:ports:input vs:bytes) (type procedure))
		(write-u8 (category r7rs:base vs:ports:output vs:bytes) (type procedure))
		
		(read-bytevector (category r7rs:base vs:ports:input vs:bytes) (type procedure))
		(read-bytevector! (category r7rs:base vs:ports:input vs:bytes) (type procedure!))
		(write-bytevector (category r7rs:base vs:ports:output vs:bytes) (type procedure))
		
		
		(char-ready? (category r7rs:base vs:ports:input vs:strings vs:characters) (type predicate))
		(peek-char (category r7rs:base vs:ports:input vs:strings vs:characters) (type procedure))
		(read-char (category r7rs:base vs:ports:input vs:strings vs:characters) (type procedure))
		(write-char (category r7rs:base vs:ports:output vs:strings vs:characters) (type procedure))
		
		(read-string (category r7rs:base vs:ports:input vs:strings) (type procedure))
		(read-line (category r7rs:base vs:ports:input vs:strings) (type procedure))
		
		
		(newline (category r7rs:base vs:ports:output vs:bytes vs:strings) (type procedure))
		(flush-output-port (category r7rs:base vs:ports:output) (type procedure))
		
		
		(read (category r7rs:read vs:ports:input vs:ports:values) (type procedure))
		(write (category r7rs:write vs:ports:output vs:ports:values) (type procedure))
		(write-simple (category r7rs:write vs:ports:output vs:ports:values) (type procedure))
		(write-shared (category r7rs:write vs:ports:output vs:ports:values) (type procedure))
		(display (category r7rs:write vs:ports:output vs:ports:values) (type procedure))
		
		
		(open-input-file (category r7rs:file vs:ports:input vs:ports:open) (type procedure))
		(open-binary-input-file (category r7rs:file vs:ports:input vs:ports:open) (type procedure))
		(open-output-file (category r7rs:file vs:ports:output vs:ports:open) (type procedure))
		(open-binary-output-file (category r7rs:file vs:ports:output vs:ports:open) (type procedure))
		
		(call-with-port (category r7rs:base vs:ports vs:functions) (type procedure))
		(call-with-input-file (category r7rs:file vs:ports:input vs:functions) (type procedure))
		(call-with-output-file (category r7rs:file vs:ports:output vs:functions) (type procedure))
		
		
		(eof-object (category r7rs:base vs:ports vs:globals) (type constant))
		(eof-object? (category r7rs:base vs:ports vs:globals) (type predicate))
		
		
		
		
		(file-exists? (category r7rs:file vs:file-system) (type procedure))
		(delete-file (category r7rs:file vs:file-system) (type procedure))
		
		
		
		
		(exit (category r7rs:process-context) (type procedure))
		(emergency-exit (category r7rs:process-context) (type procedure))
		
		(command-line (category r7rs:process-context) (type procedure))
		(get-environment-variable (category r7rs:process-context) (type procedure))
		(get-environment-variables (category r7rs:process-context) (type procedure))
		
		(current-second (category r7rs:time) (type procedure))
		(current-jiffy (category r7rs:time) (type procedure))
		(jiffies-per-second (category r7rs:time) (type procedure))
		
		
		
		
		(char? (category r7rs:base vs:characters vs:types) (type type-predicate))
		
		(char=? (category r7rs:base vs:characters vs:comparisons vs:equivalence) (type comparator=))
		(char<? (category r7rs:base vs:characters vs:comparisons) (type comparator<))
		(char>? (category r7rs:base vs:characters vs:comparisons) (type comparator>))
		(char<=? (category r7rs:base vs:characters vs:comparisons) (type comparator<=))
		(char>=? (category r7rs:base vs:characters vs:comparisons) (type comparator>=))
		
		(char-ci=? (category r7rs:char vs:characters vs:comparisons vs:equivalence) (type comparator=))
		(char-ci<? (category r7rs:char vs:characters vs:comparisons) (type comparator<))
		(char-ci>? (category r7rs:char vs:characters vs:comparisons) (type comparator>))
		(char-ci<=? (category r7rs:char vs:characters vs:comparisons) (type comparator<=))
		(char-ci>=? (category r7rs:char vs:characters vs:comparisons) (type comparator>=))
		
		(char->integer (category r7rs:base vs:characters) (type converter))
		(integer->char (category r7rs:base vs:characters) (type converter))
		(digit-value (category r7rs:char vs:characters) (type converter))
		
		(char-alphabetic? (category r7rs:char vs:characters) (type predicate))
		(char-upper-case? (category r7rs:char vs:characters) (type predicate))
		(char-lower-case? (category r7rs:char vs:characters) (type predicate))
		(char-numeric? (category r7rs:char vs:characters) (type predicate))
		(char-whitespace? (category r7rs:char vs:characters) (type predicate))
		
		(char-upcase (category r7rs:char vs:characters) (type procedure))
		(char-downcase (category r7rs:char vs:characters) (type procedure))
		(char-foldcase (category r7rs:char vs:characters) (type procedure))
		
		
		
		(procedure? (category r7rs:base vs:functions vs:types) (type type-predicate))
		
		(apply (category r7rs:base vs:functions) (type procedure))
		
		(values (category r7rs:base vs:functions vs:values) (type constructor))
		(call-with-values (category r7rs:base vs:functions vs:values) (type procedure))
		
		
		
		
		(error-object? (category r7rs:base vs:errors) (type type-predicate))
		(read-error? (category r7rs:base vs:errors) (type predicate))
		(file-error? (category r7rs:base vs:errors) (type predicate))
		
		(error (category r7rs:base vs:errors) (type constructor))
		(error-object-message (category r7rs:base vs:errors) (type accessor))
		(error-object-irritants (category r7rs:base vs:errors) (type accessor))
		
		
		
		
		(guard (category r7rs:base vs:errors vs:evaluator) (type syntax)
			(syntax-rules
					(
						(variable identifier)
						(else literal)
						(clause-condition expression)
						(clause-expression expression)
						(clause pattern
							(clause-condition)
							(clause-condition clause-expression ...)
							(else clause-expression ...))
						(guarded-expression expression))
				(_ (variable clause ...) guarded-expression ...)
			))
		
		(with-exception-handler (category r7rs:base vs:errors vs:evaluator) (type procedure))
		
		(raise (category r7rs:base vs:errors vs:evaluator) (type procedure))
		(raise-continuable (category r7rs:base vs:errors vs:evaluator vs:unsupported) (type procedure))
		
		
		
		
		(parameterize (category r7rs:base vs:parameters) (type syntax)
			(syntax-rules
					(
						(parameter expression)
						(initializer expression)
						(parameters pattern
							()
							((parameter initializer) ...))
						(expression expression))
				(_ parameters)
				(_ parameters expression ...)
			))
		
		(make-parameter (category r7rs:base vs:parameters) (type constructor))
		
		(current-input-port (category r7rs:base vs:parameters) (type parameter))
		(current-output-port (category r7rs:base vs:parameters) (type parameter))
		(current-error-port (category r7rs:base vs:parameters) (type parameter))
		
		(with-input-from-file (category r7rs:file vs:parameters vs:functions) (type procedure))
		(with-output-from-file (category r7rs:file vs:parameters vs:functions) (type procedure))
		
		
		
		
		(delay (category r7rs:lazy vs:promises vs:evaluator) (type syntax)
			(syntax-rules
					((expression expression))
				(_ expression)
			))
		
		(delay-force (category r7rs:lazy vs:promises vs:evaluator) (type syntax)
			(syntax-rules
					((expression expression))
				(_ expression)
			))
		
		(promise? (category r7rs:lazy vs:promises vs:evaluator) (type type-predicate))
		(make-promise (category r7rs:lazy vs:promises vs:evaluator) (type constructor))
		(force (category r7rs:lazy vs:promises vs:evaluator) (type procedure))
		
		
		
		
		(eval (category r7rs:eval vs:evaluator vs:unsupported) (type procedure))
		(environment (category r7rs:eval vs:evaluator vs:unsupported) (type procedure))
		
		(interaction-environment (category r7rs:r5rs r7rs:repl vs:evaluator vs:unsupported) (type procedure))
		(scheme-report-environment (category r7rs:r5rs vs:evaluator vs:unsupported) (type procedure))
		(null-environment (category r7rs:r5rs vs:evaluator vs:unsupported) (type procedure))
		
		
		
		
		(call-with-current-continuation (category r7rs:base vs:continuations vs:unsupported) (type procedure) (alias call/cc))
		(dynamic-wind (category r7rs:base vs:continuations vs:unsupported) (type procedure))
		
		
		
		
		(cond-expand (category r7rs:base vs:compiler vs:unsupported) (type syntax))
		(features (category r7rs:base vs:evaluator vs:compiler vs:unsupported) (type procedure))
		
		(include (category r7rs:base vs:compiler vs:unsupported) (type syntax))
		(include-ci (category r7rs:base vs:compiler vs:unsupported) (type syntax))
		
		(import (category r7rs:base vs:compiler vs:unsupported) (type syntax))
		
		(load (category r7rs:load vs:compiler vs:unsupported) (type procedure))
		
		
		
		
	)
	
	
	(types
		
		(any (category r7rs-x:types))
		
		(null (category r7rs-x:types-disjoint) (parent any) (predicate null?))
		
		(boolean (category r7rs-x:types-disjoint) (parent any) (predicate boolean?))
		(number (category r7rs-x:types-disjoint) (parent any) (predicate number?))
		
		(symbol (category r7rs-x:types-disjoint) (parent any) (predicate symbol?))
		
		(character (category r7rs-x:types-disjoint) (parent any) (predicate char?))
		(string (category r7rs-x:types-disjoint) (parent any) (predicate string?))
		
		(bytevector (category r7rs-x:types-disjoint) (parent any) (predicate bytevector?))
		
		(vector (category r7rs-x:types-disjoint) (parent any) (predicate vector?))
		
		(pair (category r7rs-x:types-disjoint) (parent any) (predicate pair?))
		
		(port (category r7rs-x:types-disjoint) (parent any) (predicate port?))
		(eof-object (category r7rs-x:types-disjoint) (parent any) (predicate eof-object?))
		
		(procedure (category r7rs-x:types-disjoint) (parent any) (predicate procedure?))
		
	)
	
	
	
	
)
