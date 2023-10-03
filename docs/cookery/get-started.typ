#import "/docs/cookery/book.typ": book-page

#show: book-page.with(title: "Get Started")

= Get Started

In this chapter, you will learn how to #link(<installation>)[install] your typst.ts, #link(<import>)[import] it to your project, and run the #link(<run-compiler>)[compiler] or #link(<run-renderer>)[renderer] module with typst.ts.

== Installation <installation>

To get functionality of #link("https://typst.app")[typst], typst.ts provides a core JavaScript library along with two Wasm library:
- `typst.ts`: the core JavaScript library which wraps Wasm modules with more friendly JavaScript APIs.
- `typst-ts-renderer`: a Wasm module that provides rendering functionality.
- `typst-ts-web-compiler`: a Wasm module that provides compilation functionality.

You can install them via #link("https://www.npmjs.com/")[npm] or #link("https://yarnpkg.com/")[Yarn] separately (npm as an example):

```bash
npm install @myriaddreamin/typst.ts
# Optional: if you want to run a typst renderer.
npm install @myriaddreamin/typst-ts-renderer
# Optional: if you want to run a typst compiler.
npm install @myriaddreamin/typst-ts-web-compiler
```

== Import typst.ts to your project <import>

#let easy_color = green.darken(25%)
#let hard_color = red.darken(25%)

There are several ways to setup typst.ts. The difficulty of each approach is evaluated by how many resources you need to configure and whether you need to be familiar with #text(fill: easy_color, [JavaScript]) or #text(fill: hard_color, [Rust]).

#let difficult-easy = text(fill: easy_color, "easy")
#let difficult-medium = text(fill: orange.darken(25%), "medium")
#let difficult-hard = text(fill: hard_color, "hard")

- #box(link(<approach-bundle>)[Approach 1]), difficulty: #difficult-easy
  
  Use a bundled javascript file along with wasm modules.

- #box(link(<approach-node-lib>)[Approach 2]), difficulty: #difficult-easy

  Use typst.ts as a library in Node.js. 

- #box(link(<approach-ts-lib>)[Approach 3]), difficulty: #difficult-medium:

  Use typst.ts as a library in browser (for TypeScript users).

- #box(link(<approach-js-lib>)[Approach 4]), difficulty: #difficult-medium:

  Use typst.ts as a library in browser (for JavaScript users).

- #box(link(<approach-ts-lib-from-source>)[Approach 5]), difficulty: #difficult-hard:

  Use typst.ts with customized renderer/compiler modules.

#line(length: 100%)

=== Use a bundled javascript file along with wasm modules. <approach-bundle>
#let bundle-example = link("https://github.com/Myriad-Dreamin/typst.ts/blob/main/packages/typst.ts/index.html")[Single HTML file]

Difficulty: #difficult-easy, Example: #bundle-example

You can include a single bundle file of `@myriaddreamin/typst.ts` in your html file and load needed wasm modules via `fetch` api.

```html
<script type="module"
  src="/core/dist/esm/main.bundle.js"></script>
<script>
let renderModule = window.TypstRenderModule;
let renderPlugin = 
  renderModule.createTypstRenderer(pdfjsLib);
renderPlugin
  .init({
   getModule: () => fetch(
    'path/to/typst_ts_renderer_bg.wasm'),
  })
  .then(async () => {
    console.log('renderer initialized', renderPlugin);
    // do something with renderPlugin
  });
</script>
```

See #bundle-example for a complete example.

=== Use typst.ts as a library in Node.js. <approach-node-lib>

Difficulty: #difficult-easy

You can import typst.ts as a library:

```typescript
import { createTypstRenderer } from
  '@myriaddreamin/typst.ts/dist/esm/renderer.mjs';

const renderer = createTypstRenderer();
renderer.init({}).then(...);
```

There are several templates for developing typst.ts with Node.js:

- #link("https://github.com/Myriad-Dreamin/typst.ts/tree/main/packages/templates/node.js")[Use renderer, with typescript configured with:]
  ```json { "moduleResolution": "Node" }``` or #linebreak()
  ```json { "moduleResolution": "Node10" }```
- #link("https://github.com/Myriad-Dreamin/typst.ts/tree/main/packages/templates/node.js-next")[Use renderer, with typescript configured with:]
  ```json { "moduleResolution": "Node16" }``` or #linebreak()
  ```json { "moduleResolution": "NodeNext" }```
- #link("https://github.com/Myriad-Dreamin/typst.ts/tree/main/packages/templates/ts-node")[Use ts-node, with typescript configured with:]
  ```json { "moduleResolution": "Node" }``` or #linebreak()
  ```json { "moduleResolution": "Node10" }```
- #link("https://github.com/Myriad-Dreamin/typst.ts/tree/main/packages/templates/ts-node-next")[Use ts-node, with and typescript configured with:]
  ```json { "moduleResolution": "Node16" }``` or #linebreak()
  ```json { "moduleResolution": "NodeNext" }```
- #link("https://github.com/Myriad-Dreamin/typst.ts/tree/main/packages/templates/node.js-compiler-next")[Use compiler, with typescript configured with:]
  ```json { "moduleResolution": "Node16" }``` or #linebreak()
  ```json { "moduleResolution": "NodeNext" }```

=== Use typst.ts as a library in browser (for TypeScript users). <approach-ts-lib>

Difficulty: #difficult-medium

You can import typst.ts as a library:

```typescript
import { createTypstRenderer } from
  '@myriaddreamin/typst.ts/dist/esm/renderer.mjs';

const renderer = createTypstRenderer();
renderer.init({
   getModule: () => fetch(...),
  }).then(...);
```

=== Use typst.ts as a library in browser (for JavaScript users). <approach-js-lib>

Difficulty: #difficult-medium

Please ensure your main file is with `mjs` extension so that nodejs can recognize it as an es module.

```shell
node main.mjs
```

=== Use typst.ts with customized renderer/compiler modules. <approach-ts-lib-from-source>

Difficulty: #difficult-hard

People familiar with rust can develop owned wasm modules with typst.ts so that they can eliminate unnecessary features and reduce the size of the final bundle. For example, if you want to build a renderer module that only supports rendering svg, you can build it like this:

```shell
wasm-pack build --target web --scope myriaddreamin -- --no-default-features --features render_svg
```

#line(length: 100%)

=== Configure path to wasm module

You may have modified the path to wasm module or rebuilt the wasm module for your own purpose. In this case, you need to configure the path to wasm module. There is a `getModule` option in `init` function that you can use to configure the path to wasm module:

```ts
renderer.init({
  getModule: () => __wasm_module_resource__,
}).then(...);
```

You can load `__wasm_module_resource__` in several ways:

```ts
// from url
const getModule = () => 'http://...';
// from http request
const getModule = () => fetch('http://...');
// from local file
const { readFileSync } = require('fs');
const getModule = () => new Uint8Array(readFileSync('path/to/wasm/module').buffer);
// instantiated wasm module
const getModule = () => WebAssembly.instantiate(/* params */);
// asynchronously
const getModule = async () => {/* above four ways */};
```

== Configure and run compiler <run-compiler>

- Configure font resources

- Configure access model

- Configure package registry

=== Precompile with `typst-ts-cli`

=== Build a compilation service in rust

== Configure and run renderer <run-renderer>

- Configure font resources, same as compiler.

=== Full Code Listing
