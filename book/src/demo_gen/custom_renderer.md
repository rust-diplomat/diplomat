# Making Your Own Renderer

Inside of `index.mjs`, demo_gen outputs an object called `RenderInfo` that points to all the functions demo_gen has created for the purposes of demonstration.

`RenderInfo` gives you the function to call directly, as well as the required parameters needed for each function in order.

This is meant to slot in to almost any Javascript solution with ease, but if there's an issue with `RenderInfo`s setup that is not quite compatible with your solution, please [open an issue](https://github.com/rust-diplomat/diplomat/issues/new?labels=B-demo_gen).

The exact structure of `RenderInfo` is available in the demo_gen [design docs](https://github.com/rust-diplomat/diplomat/blob/main/docs/demo_gen.md#step-two-constructing-renderinfo).