# latex-table-generation
A rust project to generate Latex Table.

## Stack technic
Use rust as back of a web app with [WebAssembly](https://www.rust-lang.org/fr/what/wasm) and trunk server.


### Compilation to wasm and serve

use `trunk serve` to serve the app, nedd target `wasm32`, added with `rustup target add wasm32-unknown-unknown` (maybe already added with trunk in the flake, not sure)

## Persistence

We doesn't persist the app after reset, it is a mess with [navigator security](https://stackoverflow.com/questions/49539306/firefox-service-worker-securityerror-domexception-the-operation-is-insecure).
