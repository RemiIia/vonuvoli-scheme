

<a id='definition__r7rs__output-port_3f'></a>

# `output-port?` -- `r7rs` Definition


<a id='definition__r7rs__output-port_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__r7rs__output-port_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((output-port) -> (true))`
   * input: a value of type [`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((port) -> (false))`
   * input: a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any) -> (false))`
   * input: a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any ...) -> (boolean))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` (i.e. variadic);
   * output: a value of type [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);
   * requires: `vonuvoli`


<a id='definition__r7rs__output-port_3f__exports'></a>

#### Exports

 * [`scheme:base`](../../r7rs/exports/scheme_3a_base.md#export__r7rs__scheme_3a_base) -- `(scheme base)`;


<a id='definition__r7rs__output-port_3f__exports-recursive'></a>

#### Exports recursive

 * [`scheme`](../../r7rs/exports/scheme.md#export__r7rs__scheme) -- `(scheme)`;


<a id='definition__r7rs__output-port_3f__description'></a>

#### Description

> Please refer to [`port?`](../../r7rs/definitions/port_3f.md#definition__r7rs__port_3f).


<a id='definition__r7rs__output-port_3f__referenced-types'></a>

#### Referenced-types

 * [`output-port`](../../r7rs/types/output-port.md#type__r7rs__output-port);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);
 * [`boolean`](../../r7rs/types/boolean.md#type__r7rs__boolean);


<a id='definition__r7rs__output-port_3f__categories'></a>

#### Categories

 * [`vs:ports:output`](../../r7rs/categories/vs_3a_ports_3a_output.md#category__r7rs__vs_3a_ports_3a_output);


<a id='definition__r7rs__output-port_3f__categories-recursive'></a>

#### Categories recursive

 * [`vs:ports`](../../r7rs/categories/vs_3a_ports.md#category__r7rs__vs_3a_ports);
 * [`vs`](../../r7rs/categories/vs.md#category__r7rs__vs);

----

Goto: [library](../../r7rs/_index.md#library__r7rs), [categories](../../r7rs/categories/_index.md#toc__r7rs__categories), [exports](../../r7rs/exports/_index.md#toc__r7rs__exports), [definitions](../../r7rs/definitions/_index.md#toc__r7rs__definitions), [types](../../r7rs/types/_index.md#toc__r7rs__types), [appendices](../../r7rs/appendices/_index.md#toc__r7rs__appendices), other [libraries](../../_libraries.md#toc__libraries).

----
