# latex-table-generation

A rust project to generate Latex Table.

# Technical stack

Use rust as back of a web app with [WebAssembly](https://www.rust-lang.org/fr/what/wasm) and NPM server.

### Compilation to wasm and serve with npm

`wasm-pack build --target bundler`: Used to build the sources in the pkg/ folder. Do it after every rust change.

`npm install`: Install all dependencies declared in `package.json`, including the custom `latex-table-generation` WASM Rust library.

`npm run serve`: Run the dev server. Need to have compiled and installed dependencies with `npm install` before.
