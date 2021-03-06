

<a id='definition__vonuvoli__textual-port_3f'></a>

# `textual-port?` -- `vonuvoli` Definition


<a id='definition__vonuvoli__textual-port_3f__kind'></a>

#### Kind

`predicate`;


<a id='definition__vonuvoli__textual-port_3f__extends'></a>

#### Extends

 * [`textual-port?`](../../r7rs/definitions/textual-port_3f.md#definition__r7rs__textual-port_3f) (from [`r7rs`](../../r7rs/_index.md#library__r7rs));


<a id='definition__vonuvoli__textual-port_3f__procedure-signature'></a>

#### Procedure signature

Procedure variants:
 * `((textual-port |1...|) -> (true))`
   * inputs:
     * a value of type [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
     * `...` -- at least one time;
   * output: a value of type [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * `((port |1...|) -> (false))`
   * inputs:
     * a value of type [`port`](../../r7rs/types/port.md#type__r7rs__port);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * `((any |1...|) -> (false))`
   * inputs:
     * a value of type [`any`](../../r7rs/types/any.md#type__r7rs__any);
     * `...` -- at least one time;
   * output: a value of type [`false`](../../r7rs/types/false.md#type__r7rs__false);


<a id='definition__vonuvoli__textual-port_3f__exports'></a>

#### Exports

 * [`vs:io`](../../vonuvoli/exports/vs_3a_io.md#export__vonuvoli__vs_3a_io) -- `(vonuvoli io)`;


<a id='definition__vonuvoli__textual-port_3f__referenced-types'></a>

#### Referenced-types

 * [`textual-port`](../../r7rs/types/textual-port.md#type__r7rs__textual-port);
 * [`true`](../../r7rs/types/true.md#type__r7rs__true);
 * [`port`](../../r7rs/types/port.md#type__r7rs__port);
 * [`false`](../../r7rs/types/false.md#type__r7rs__false);
 * [`any`](../../r7rs/types/any.md#type__r7rs__any);


<a id='definition__vonuvoli__textual-port_3f__categories'></a>

#### Categories

 * [`vs:r7rs`](../../vonuvoli/categories/vs_3a_r7rs.md#category__vonuvoli__vs_3a_r7rs);
 * [`vs:ports`](../../vonuvoli/categories/vs_3a_ports.md#category__vonuvoli__vs_3a_ports);

----

Goto: [library](../../vonuvoli/_index.md#library__vonuvoli), [categories](../../vonuvoli/categories/_index.md#toc__vonuvoli__categories), [exports](../../vonuvoli/exports/_index.md#toc__vonuvoli__exports), [definitions](../../vonuvoli/definitions/_index.md#toc__vonuvoli__definitions), other [libraries](../../_libraries.md#toc__libraries).

----

