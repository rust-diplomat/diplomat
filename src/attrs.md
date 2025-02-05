# Customizing via backend attributes

Diplomat supports a number of "backend attributes" that allow one to customize the backend-side code generation of a particular API. This allows for more expressivity than that afforded by Diplomat's relatively constrained type system, including things like the ability to rename types, namespace types, or mark types as iterators.


## Configurability

All backend attributes are applied via the `#[diplomat::attr(...)]` directive. As its first parameter, it requires a configuration specification, which can be:

 - `*`, meaning "all backends"
 - `auto`, explained below
 - A backend/language name: (`c`, `cpp`, `js`, `dart`, `kotlin`, `demo`, ...). Note that a backend may accept multiple names, for example the `demo` backend both accepts attributes marked for the `js` and `demo` backends.
 - A `supports` query, like `supports = constructors`. A full list can be found on the [`BackendAttrSupport`] type, but furthermore the pages on individual attributes will specifically list configuration options relevant to that attribute.
 - `not(...)` containing a configuration specification
 - `any(...)` containing one or more comma separated configuration specifications 
 - `all(...)` containing one or more comma separated configuration specifications 


So, for example, to  rename a type for everyone, one may use something like `#[diplomat::attr(*, rename = "FancyNewName")]` on the type. However if the rename is only intended for a select set of backends, one may do something like `#[diplomat::attr(any(js, cpp), rename = "FancyNewName")]`.


Similarly, if one wishes to only expose a type to backends that support iterators, one may do something like `#[diplomat::attr(not(supports = iterators), disable)]`.


A lot of attributes (basically all of them except for `rename` and `disable`) represent features that need not be present in all backends. For example, not every backend wishes to, or can, support iteration, or namespacing, or accessors. By default when an unsupported attribute is passed to a backend, Diplomat will error as a lint. However, this has the unfortunate effect of sprinkling the code with a lot of stuff that looks like `#[diplomat::attr(supports = iterators, iterator)]`.

To aid in that, `auto` can be used instead. `auto` is roughly equivalent to `supports = whichever attribute this is attempting to apply`.



## When to configure

Besides for reasons of differing backend support, Diplomat's attribute configuration is useful for providing a more idiomatic, tailored experience for individual languages. For example, in some languages (like C++) adding a field to a struct accepted as a parameter in a function would be a breaking change, but in languages like JS that change can be made without any breakage if the new field is nullable. In such a case, the appropriate mixing of language-specific renames can lead to a split v1/v2 API in C++ whilst JS gets a smoother experience with no need for versioning.




  [`BackendAttrSupport`]: https://docs.rs/diplomat_core/latest/diplomat_core/hir/struct.BackendAttrSupport.html