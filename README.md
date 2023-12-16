# usaddress-rs

A Rust port of [usaddress](https://github.com/datamade/usaddress), a Python library for parsing US-based postal addresses. Uses the same `crfsuite` model under-the-hood.

## TODO

- [ ] lib
  - [ ] stress test tokenizer
  - [ ] impl `tag()` - clean up tokens before returning
  - [ ] proper AST for annotated results on original string?
  - [ ] stress test features with Python lib
  - [ ] snapshot tests
  - [ ] new custom model niput
- [ ] cli
  - [ ] use clap?
  - [ ] maybe read in a CSV? idk
- [ ] sqlite
  - [ ] `usaddress_parts()` table function
  - [ ] `usaddress_part(input, part)`
  - [ ] update base model
  - [ ] packaging, distirubtion, etc
- [ ] wasm
  - [ ] better UI
  - [ ] publish npm package?
- [ ] overall
  - [ ] find messy address dataset for examples/demos/benchmarks (fastfec?)
  - [ ] benchmarks against original Python library
