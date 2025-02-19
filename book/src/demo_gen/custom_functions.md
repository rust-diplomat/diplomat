# Making Custom Functions

## What is a custom function?

demo_gen tends towards automagical configuration. demo_gen will do its best to take Rust functions and convert them into JS output.

But there arise situations where we want to create our own custom Javascript functions to demonstrate our library's capabilities to the user, then add them to demo_gen's output. This may be the case if you want to demonstrate functionality that is more involved than demo_gen's automagical work.

### Example
Let's look at the [quickstart](quickstart.md) repository for an example.

We only have one function exposed: `get_add_str(left : u32, right: u32)`.

What if we have variables `a`, `b`, and `c`, and we want to show the user the results of calling:

`get_add_str(a, b)` and `get_add_str(b, c)`?

We can do this without adding a new binding, through the use of a custom Javascript function.

## \#\[diplomat::demo(custom_func="...")\]

`#[diplomat::demo(custom_func="filename.mjs")]` can be added above any `struct` definition. demo_gen will search for files relative to `lib.rs`, and add the contents of `filename.mjs` to its output.

Then demo_gen will import the default export of `filename.mjs`, and append it to the list of [RenderInfo](https://github.com/rust-diplomat/diplomat/blob/main/docs/demo_gen.md#step-two-constructing-renderinfo) termini.

### Example
So, first we create a file called `adder_custom.mjs` in the same folder as `adder_bindings/src/lib.rs`:

```js
// adder_bindings/src/adder_custom.mjs
import { lib } from "./index.mjs";
export default {
    "AddThreeVariables": {
        func: (a, b, c) => { return lib.AddResult.getAddStr(a, b) + " and " + lib.AddResult.getAddStr(b, c); },
        funcName: "Add a + b, b + c",
        parameters: [
            {
                name: "a",
                type: "number"
            },
            {
                name: "b",
                type: "number"
            },
            {
                name: "c",
                type: "number"
            }
        ]
    }
}
```

Then we make sure to link `adder_custom.mjs` in `lib.rs`:

```rs
// adder_bindings/src/lib.rs
#[diplomat::bridge]
mod ffi {
    use std::fmt::Write;

	#[diplomat::opaque]
	#[diplomat::rust_link(basic_adder, Mod)]
    #[diplomat::demo(custom_func="adder_custom.mjs")]
	pub struct AddResult;

	impl AddResult {
		pub fn get_add_str(left : u32, right : u32, write: &mut DiplomatWrite) {
			write.write_str(&format!("{}", basic_adder::add(left, right))).unwrap();
			write.flush();
		}
	}
}
```

And our exported object is then added to `RenderInfo`s list of render termini, and is evaluated by the renderer accordingly!

If you [regenerate the bindings and start the web server](https://rust-diplomat.github.io/book/demo_gen/quickstart.html#getting-started), you should see `Add a + b, b + c` in the list of functions.