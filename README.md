# usaddress-rs

A Rust port of [usaddress](https://github.com/datamade/usaddress), a Python library for parsing US-based postal addresses. Uses the same `crfsuite` model under-the-hood.

Not done yet - a big work in progress! Once done, there will be:

- A low-level Rust library implementing a similar API to the Python one
- A SQLite extension
- A WASM library for running in the browser
- A small CLI tool
