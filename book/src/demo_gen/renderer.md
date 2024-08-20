# Configuring the Default Renderer

demo_gen comes bundled with an HTML renderer to make getting started with creating demo_gen output to be as fast as possible. The default renderer is also designed to be highly customizable for your own preferences or front ends.

The front end renderer uses [Web Components](https://developer.mozilla.org/en-US/docs/Web/API/Web_components), which are natively supported by most browsers. For this reason, it should be very portable into other front end systems like Svelte or React. However, if you're dead set on a solution that works even *better* for your front end of choice, you should read [making your own renderer](./custom_renderer.md).

For more on how the default renderer works, you can read our [design doc](https://github.com/rust-diplomat/diplomat/blob/main/docs/design_doc.md).

Regardless, let's discuss some ways you can customize the default renderer to your liking.

## template.html

`rendering/template.html` represents a list of templates that demo_gen's default renderer will use 

demo_gen will automatically generate `template.html` in the rendering folder. There is nothing that ties `template.html` to this folder specifically however; you can copy, modify, and link to a changed `template.html` file for custom HTML, JS, and CSS.

For instance, this is one template we've overridden in the ICU4X repo to take advantage of Bootstrap:

```html
<template id="terminus">
<link rel="stylesheet" href="dist/index.css"/>
<div class="vstack gap-2">
	<h1><slot name="func-name"></slot></h1>
	<slot name="parameters"></slot>
	<button type="submit" class="btn btn-primary" data-submit>Submit</button>
	<div class="card">
		<div class="card-header">Output</div>
		<div class="card-body">
			<p><slot name="output">Output Shown Here</slot></p>
		</div>
	</div>
</div>
</template>
```

For `<template>` tags, we hook into events by looking for `data-*` attributes, which have some of the following properties:

- `data-submit` tells the attached element to await a press before attempting to run demo_gen code (only works for the `#terminus` tag).
- `data-oninput` tells the attached element to listen for the `oninput` event and save the user's input on this element for submission.

If you're on the [quickstart](quickstart.md) repository, you might try copying `template.html` out of the rendering folder and modifying it yourself to include your own stylesheets.

> [!NOTE]
> Because the renderer uses the Web Components API, stylesheets need to be linked inside of each `<template>` tag.  

## runtime.mjs

This is simply a wrapper for the underlying `rendering/rendering.mjs`, which contains most of the logic for taking `<template>` tags and transforming them into 

The expected end result of `runtime.mjs` is to create a `TerminusRender` object from `rendering.mjs`, and append it to the HTML.

If you are interested in overriding the underlying Javascript more thoroughly, reading the documentation [on writing your own custom renderer](custom_renderer.md) is recommended. Otherwise, you will mostly be interested in overwriting the `evaluateExternal` parameter, which looks something like this:

```js
(param, updateParamEvent) => {
	console.error(`Unrecognized parameter type ${param}`);
}
```

If you've flagged anything with the [external](attributes.md#diplomatdemoexternal) attribute, you can check for parameters that Diplomat cannot evaluate on its own and provide these yourself with the `updateParamEvent(updatedParamValue)` callback, containing the value of the parameter that is required.

> [!TIP]
> `evaluateExternal` is only called once on creation, so if you're planning on updating a param more than once, you should save a dictionary of `updateParamEvent` callbacks somewhere for future reference.

For example, in the ICU4X demo, we look for the DataProvider parameter and provide it from a compiled set of data:

```js
let dataProvider = DataProvider.compiled();
let evaluateExternal = (param, updateParamEvent) => {
    if (parameter.type === "DataProvider") {
        updateParamEvent(dataProvider);
    } else {
        console.error(`Unrecognized parameter type ${param}`);
    }
};
```

## index.html

demo_gen currently doesn't provide an `index.html` file for you, as even with the default renderer your file structure can vary wildly. It is up to the user to provide their own additional `.html` files.

If you're looking to get into output right away: you can access any function from the default renderer by opening `template.html` from your webserver with the URL `/renderer/template.html?func=TypeName.functionName`.

Here's the current script that [the quickstart](quickstart.md) has to list all possible function names:

```js
import { RenderInfo } from "./demo_gen/index.mjs";

Object.values(RenderInfo.termini).forEach((t) => {
	let a = document.createElement("li");
	a.innerHTML = `<a href="demo_gen/rendering/template.html?func=${t.funcName}">${t.funcName}</a>`;
	document.getElementById("links").appendChild(a);
});
```