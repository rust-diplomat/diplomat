Example of the `demo_gen` backend working with a smaller version of the ICU4X library.

For a few reasons (mostly related to the CI) this folder is not self-contained. It depends on the JS bindings, and for this reason must be viewed through a webserver from the root of the diplomat repo.

If you're looking for a more simple, self-contained example, please see the [quickstart repo](https://github.com/rust-diplomat/demo-gen-quickstart).

If you're looking for an example of how to bundle multiple dependencies through say, webpack, please see [the ICU4X repository](https://github.com/unicode-org/icu4x/tree/main/tutorials/web-demo).

To view the example in action, run:

```bash
npm install
npm run start
```

Then navigate to the `demo_gen` folder in the HTTP Server that opens.