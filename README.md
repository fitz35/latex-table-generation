# latex-table-generation
A rust project to generate Latex Table.

## Stack technic
Use rust as back of a web app with [WebAssembly](https://www.rust-lang.org/fr/what/wasm) and trunk server.


### Compilation to wasm and serve with npm

Use the command `wasm-pack build --target bundler` to build the sources in the pkg/ folder, do it after every rust change.
Then, use `npm run serve` in the `site/` folder to serve the app.