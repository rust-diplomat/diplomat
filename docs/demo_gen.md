# demo_gen design doc
demo_gen is a Diplomat Backend that relies on Diplomat's JS backend to create examples of using your library's FFI bindings in Javascript.

demo_gen is designed to work with minimal configuration, as well as support a high amount of customization so that you can adjust its appearance and functionality to your liking.

This design doc covers nearly everything about the inner workings of demo_gen. If you're curious as to how you can use demo_gen, you'll want to check the [Diplomat docs](https://rust-diplomat.github.io/book/demo_gen/intro.html).

## Design Goals
- demo_gen should show how functions in your library work.
  - Users learn by example, and demo_gen should be a simple way to create those examples.
  - We do not aim for completeness of demonstration, but a sample platter or charcuterie board of your library. 
- demo_gen should have a short setup time.
  - It should take minimal effort to get an example up and running.
  - We want to minimize time to setup, but we can't eliminate it entirely. See the [docs](https://rust-diplomat.github.io/book/demo_gen/renderer.html) for more on why setup time is not instantaneous.
- demo_gen should be maximally customizable.
  - Not everyone uses the same front-end, not everyone has the same idea of what a demo should look like.
  - Nearly every aspect of what demo_gen outputs and how it looks should be configurable.

### Example Part 0
A good use case for demo_gen is the [ICU4X internationalization library](https://github.com/unicode-org/icu4x/).

ICU4X makes up a large number of different internationalization components. Potential users may wish to see what ICU4X is capable of offering without the hassle of compiling their own packages. A simple web link with a list of possible functions is the best answer for these kinds of users.

This is exactly the kind of problem that demo_gen aims to solve.

For these example sections we will use the simplified ICU4X functions from the example/ folder that is in the Diplomat repository.

For instance, let's say I want to know how the `FixedDecimalFormatter.formatDecimal` function works. An inquisitive user should be able to put in a number in a webpage, and have it give me a formatted decimal in the given locale I set.

We could set up this functionality manually, but `demo_gen` has the capabilities to do this for us. It can read the `FixedDecimalFormatter.formatDecimal` function and create an HTML page with all the needed inputs and underlying logic that I might normally have to do myself.

Let's explore how this happens.

## How demo_gen works
Recommended you read the [diplomat design doc](./design_doc.md) for further background on the processes demo_gen is built on.

To start: demo_gen is split into three parts: attributes, the markup generator, and the front end renderer.

### Attributes

demo_gen uses the `#[diplomat::demo]` attribute for special configuration.

Right now, this attribute is unique to the demo_gen backend. But in case anyone else decides to create their own non-JS demo generation backends, `#[diplomat::demo]` is backend agnostic. We look for them in the AST, and evaluate them in the HIR.

Full documentation of available attributes is available at the [docs](https://rust-diplomat.github.io/book/demo_gen/intro.html).

#### Example Part 1

You may notice some `#[diplomat::demo]` attributes present in the examples/ folder:

```rs
#[diplomat::demo(default_constructor)]
pub fn try_new(
	locale: &Locale,
	provider: &DataProvider,
	options: FixedDecimalFormatterOptions,
) -> Result<Box<FixedDecimalFormatter>, ()> { /* ... */ }
```

We'll cover this shortly.

### Render Termini
Both the markup generator and renderer are built on the concept that for demonstration purposes, any function that outputs a string (or something that can be represented with a string) can be broken down into primitive inputs for a sample of how the function works. We call these functions *Render Termini*, since they are the intended end goal (think *terminus*) for generation. The finish line for our front end renderer to show a user.

Currently, we define any Render Terminus as a function that has a `&mut DiplomatWrite` parameter[^generation]. To translate that into JS terms, it's any function that outputs a `string`.

[^generation]: demo_gen automatically finds these functions by default.

#### Example Part 2

Let's continue our examination of `FixedDecimalFormatter.formatDecimal`.

This is the Rust definition:

```rs
/// An  Fixed Decimal Format object, capable of formatting a [`FixedDecimal`] as a string.
#[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
pub struct FixedDecimalFormatter(pub icu::decimal::FixedDecimalFormatter);

impl FixedDecimalFormatter {
	/// Creates a new [`FixedDecimalFormatter`] from locale data.
	#[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
	#[diplomat::demo(default_constructor)]
	pub fn try_new(
		locale: &Locale,
		provider: &DataProvider,
		options: FixedDecimalFormatterOptions,
	) -> Result<Box<FixedDecimalFormatter>, ()> { /* ... */ }

	/// Formats a [`FixedDecimal`] to a string.
	#[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
	pub fn format_write(&self, value: &FixedDecimal, write: &mut DiplomatWrite) {
		self.0.format(&value.0).write_to(write).unwrap();
		write.flush();
	}
}
```

This is the `.d.ts` definition:

```ts
export class FixedDecimalFormatter {
    get ffiValue(): pointer;

    static tryNew(locale: Locale, provider: DataProvider, options: FixedDecimalFormatterOptions): FixedDecimalFormatter | null;

    formatWrite(value: FixedDecimal): string;
}
```

`formatWrite` outputs a `string`. So this is a Render Terminus.

### Markup Generator

Render Termini are created with the `RenderTerminusContext` struct in Rust. As we search through the HIR, we find suitable Render Termini.

#### Step One: Method Dependency Tree

From there, we need to look at each Termini's parameters. Our ultimate goal for every Render Terminus is to find the user input that constructs its output. So we look at the method parameters, and see what can be converted into user input.

So we have what's called a `MethodDependency` struct, to represent dependencies for any given method. Let's explore how each possible parameter type impacts our `MethodDependency`s fields.

Once we've fully evaluated a `MethodDependency`, we render the call of that method to Javascript and push it to a stack of similar Javascript calls, each dependent on the prior `MethodDependency`. This way, we convert the tree into a stack of calls.

The HIR splits parameters into a few different types:

##### 1. Primitive Types

`bool`s, `char`s, `int`s, etc. all have fairly common input types. They represent a parameter that our Render Terminus needs to call, but we can also gather input directly from the user because of how simple it is to grab with a checkbox or a text field.

So first, we pass these parameters to the `RenderTerminusContext::out_params` field. All `out_params` are passed directly to the JS function definition for the [renderer](#front-end-renderer) to handle.

Then, we make sure the Render Terminus' `MethodDependency` knows in what order these parameters are called with the `MethodDependency::params` field, which is just a vector of parameter call information for the specific method that `MethodDependency` represents.

##### 2. Slices

Same as primitive types, these are placed into `params`, `out_params`, and are left up to the [renderer](#front-end-renderer) to handle.

##### 3. Enums

Same as the two above, these are placed into `params`, `out_params`, and are left up to the [renderer](#front-end-renderer) to handle.

##### 4. Opaques

Opaques are tricky because by definition, they only can be created and managed by other functions. 

Because we can only create opaques from other functions, that just means we need to find a function that creates an opaque. These are mostly determined by the `#[diplomat::demo(default_constructor)]` attribute that a Diplomat user must set themselves. See [attributes](#attributes) for more. 

The nice thing about Javascript is that we can evaluate a parameter as a function before we pass it in to our Render Terminus. So if an Opaque has a valid default_constructor method, demo_gen creates a new `MethodDependency` to be called before our Render Terminus. We then recursively search through the parameters of the Opaque constructor, going through each of the HIR types and performing the same steps:

##### 5. Structs

Structs are a little different because they can be created through pure JS. So we just use the helpful `FromFields` function that comes with every non-out struct[^out]:

```js
Struct.FromFields(a, b, c, d);
```

[^out]: Out Structs cannot be generated from a simple function call, so demo_gen finds these as the result of other functions.

#### Step Two: Constructing RenderInfo

Now that we have a call stack of `MethodDependency`s, we can now create two items:
1. A Javascript function for our [renderer](#front-end-renderer) to call.
2. A JSON object for our [renderer](#front-end-renderer) to evaluate to know what Render Termini exist.

In this case, we just export an object called `RenderInfo`, with all the information any renderer will need to know about how we expect our Render Terminus to be called in Javascript.

The layout is something like:

```js
import func from "./func.mjs";

export const RenderInfo = {
	termini: {
		"functionName": {
			func: func,
			funcName: "functionName",
			parameters: [
				{
					name: "Param One",
					type: "string"
				},

				{
					name: "Param Two",
					type: "number"
				}
			]
		}
	}
};
```

The exact structure of `RenderInfo` is:

```js
export const RenderInfo = {
    termini: {
        "functionName": {
            func: jsFunctionToCall, // Always present.
            funcName: "jsFunctionToCall", // String value of the function name. Always present.
            parameters: [ // Always present.
                {
                    name: "Param Name", // Always present. Modified by `#[diplomat::demo(input(label = "..."))]`
                    type: "type_name", // Always present. Could be: string, number, boolean, Array<string>, Array<number>, Array<boolean>, or some specific JS binding class name (i.e., MyEnum).
                    // In your HTML renderer, you should generally assume that any type that is not a primitive is an enum. #[diplomat::demo(external)] parameters are also exposed here, so be prepared to include carve-outs for those exceptions.
                    defaultValue: "defaultValue" // Only present if `#[diplomat::demo(input(default_value = "..."))]` is present.
                }
            ]
        }
    }
};

```

#### Example Part 3

For `FixedDecimalFormatter.formatWrite`, let's look at the rust definition again:

```rs
pub fn format_write(&self, value: &FixedDecimal, write: &mut DiplomatWrite) { /* ... */ }
```

The function takes values of type: `&self` and `&FixedDecimal` (`&mut DiplomatWrite` is our output). We evaluate the `&self` value first. 

`FixedDecimalFormatter` is an opaque type, but someone has helpfully labelled the `try_new` function for us as a default constructor:

```rs
#[diplomat::demo(default_constructor)]
pub fn try_new(
	locale: &Locale,
	provider: &DataProvider,
	options: FixedDecimalFormatterOptions,
) -> Result<Box<FixedDecimalFormatter>, ()> { /* ... */ }
```

Now we continue to evaluate based on depth first. `Locale` is also an opaque, but it has another constructor labelled for us:

```rs
#[diplomat::attr(auto, constructor)]
pub fn new(name: &DiplomatStr) -> Box<Locale> { /* ... */ }
```

And `Locale` only takes a `&DiplomatStr`, which is something we can easily pass into `out_params`. Now we go back up one level, and see that we need a type of `&DataProvider`:

```rs
#[diplomat::demo(default_constructor)]
#[diplomat::attr(auto, named_constructor = "static")]
pub fn new_static() -> Box<DataProvider> { /* ... */ }
```

We only have a static data provider in this case, but ICU4X has more complicated data providers in the real library. This is where we might use something like `#[diplomat::demo(external)]` to tell demo_gen that we would like to be able to provide `DataProvider` ourselves when calling `formatWrite` (see [attributes](#attributes) for more).

And so we continue to recurse through all opaque methods until we finally get the following stack of JS calls:

```js
export function formatWrite(fixedDecimalFormatterLocaleName, fixedDecimalFormatterOptionsGroupingStrategy, fixedDecimalFormatterOptionsSomeOtherConfig, valueV) {
    
    let fixedDecimalFormatterLocale = new Locale(fixedDecimalFormatterLocaleName);
    
    let fixedDecimalFormatterProvider = DataProvider.newStatic();
    
    let fixedDecimalFormatterOptions = FixedDecimalFormatterOptions.fromFields({
        groupingStrategy: fixedDecimalFormatterOptionsGroupingStrategy,
        someOtherConfig: fixedDecimalFormatterOptionsSomeOtherConfig
    });
    
    let fixedDecimalFormatter = FixedDecimalFormatter.tryNew(fixedDecimalFormatterLocale,fixedDecimalFormatterProvider,fixedDecimalFormatterOptions);
    
    let value = new FixedDecimal(valueV);
    
    let out = fixedDecimalFormatter.formatWrite(value);
    

    return out;
}
```

And our `RenderInfo` for this object looks something like:

```js
export const RenderInfo = {
    termini: {
        "FixedDecimalFormatter.formatWrite": {
            func: FixedDecimalFormatterDemo.formatWrite,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimalFormatter.formatWrite",
            parameters: [
                
                {
                    name: "Locale Name",
                    type: "string"
                },
                
                {
                    name: "ICU4X Fixed Decimal Grouping Strategy",
                    type: "FixedDecimalGroupingStrategy"
                },
                
                {
                    name: "Useless Config (Ignore)",
                    type: "boolean"
                },
                
                {
                    name: "ICU4XFixedDecimal Value",
                    type: "number"
                }
                
            ]
        },
	}
};
```

Now everything's ready to be passed off to the renderer!

### Front End Renderer

We need some way to take the `RenderInfo` object and turn it into HTML elements that we can grab user input from. On top of that, we need to do things like hook JS events from those elements into our underlying code.

This is where demo_gen has the most amount of freedom in how you approach things. demo_gen is meant to be highly customizable, so long as you provide the correct `parameters` that any given Render Terminus requires.

#### Default Renderer

For ease of use, demo_gen has a built-in renderer to handle everything for developers (but it is also configurable). For more on configuration, you'll have to check the [docs](https://rust-diplomat.github.io/book/demo_gen/intro.html).

The way the default renderer works is through the [Web Components API](https://developer.mozilla.org/en-US/docs/Web/API/Web_components). That is, every Render Terminus has an associated `<render-terminus>` element that we register through Javascript, and the content of that element is determined by the `<template id="terminus">` element.

Inside the `<template id="terminus">` element, we look for the `<slot name="parameters"></slot>` for inserting input forms.

For each `RenderInfo.termini[terminus].parameters`, we create a new HTML element from the parameter's type. We have an associated `<template>` element for each primitive type for enumerators, numbers, strings, etc. Inside each `<template>`, we look for elements with `data-*` attributes to attach events to. 

Here's a brief reference of how the default renderer handles each primitive:

- `bool`s are checkboxes.
- `int`s and `float`s are `<input type="number">` elements.
- TODO: `char`s are not yet handled. 
- `enum`s are `<select>` and `<option>` elements.
- `string`s are just a string input. 
  - TODO: `DiplomatStr16` is currently not supported.
- `slice`s are represented through a comma separated list of values.
  - TODO: Only an array of strings is currently supported.

#### Example Part 4

So with a default renderer, our `formatWrite` function would look something like the following in HTML:

```html
<terminus-render>
	<span slot="func-name">FixedDecimalFormatter.formatWrite</span>
	<terminus-params slot="parameters">
		<terminus-param-string>
			<span slot="param-name">Locale Name</span>
		</terminus-param-string>
		<terminus-param-enum>
			<span slot="param-name">ICU4X Fixed Decimal Grouping Strategy</span>
		</terminus-param-enum>
		<terminus-param-boolean>
			<span slot="param-name">Useless Config (Ignore)</span>
		</terminus-param-boolean>
		<terminus-param-number>
			<span slot="param-name">ICU4XFixedDecimal Value</span>
		</terminus-param-number>
	</terminus-params>
	<span slot="output">

	</span>
</terminus-render>
```