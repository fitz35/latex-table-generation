# latex-table-generation

A rust project to generate Latex Table.

# Technical stack

Use rust as back of a web app with [WebAssembly](https://www.rust-lang.org/fr/what/wasm) and NPM server.

### Compilation to wasm and serve with npm


`wasm-pack build --target bundler`: Used to build the sources in the pkg/ folder. Do it after every rust change.
Then, use `npm run serve` in the `site/` folder to serve the app.
