# Making Custom Functions

Let's say you want to add your own custom demonstration functions for a given struct, without demo_gen automagically creating the content of JS functions for you.

This is done with the `#[diplomat::demo(custom_func="filename.mjs")]` attribute, which can be added above any `struct` definition. demo_gen will search for files relative to `lib.rs`, and add the contents of `filename.mjs` to its output.

Then demo_gen will import the default export of `filename.mjs`, and append it to the list of [RenderInfo](https://github.com/rust-diplomat/diplomat/blob/main/docs/demo_gen.md#step-two-constructing-renderinfo) termini.

Here's an example of what that looks like...

In rust:

```rs
#[diplomat::bridge]
mod ffi {
    #[diplomat::demo(custom_func="classname_demo.mjs")]
    struct ClassName;
}


```

Then in `classname_demo.mjs`, we write:

```js
export default {
    "ClassName.SampleFunctionName": {
        func: () => { alert("Hello world!"); },
        funcName: "ClassName.SampleFunctionName",
        parameters: [

        ]
    },
    "ClassName.OtherFunctionName": {
        func: (a) => { prompt(`Testing!${a}`); },
        funcName: "ClassName.OtherFunctionName",
        parameters: [
            name: "Prompt Question",
            type: "string"
        ]
    }
};
```

And our exported object is then added to `RenderInfo`s list of render termini, and is evaluated by the renderer accordingly!