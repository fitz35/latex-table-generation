# latex-table-generation
A rust project to generate Latex Table.

## Stack technic
Use rust as back of a web app with [WebAssembly](https://www.rust-lang.org/fr/what/wasm) and npm server.

## Direction

### UI

4 parts :
- at the top, the `load csv`, `save metadata` and "save latex buton.
- Left : UI button
- The rest of the screen occuped with the array and the latex code (can't edit directly the latex code)


### TODO
In order :

- [ ] display the UI
- [ ] Load the csv
- [ ] display the array
- [ ] display the latex
- [ ] Button to transpose the array
- [ ] Metadata of the array (title, label....)
- [ ] Edit of each individually case (Strong ....)
- [ ] Merge