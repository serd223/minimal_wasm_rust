# Introduction
This is a simple proof of concept project for running Rust code in browser via Webassembly without the need for complicated librares.

## Inspiration
After [watching Tsoding's streams](https://www.youtube.com/@TsodingDaily) about his project [koil](https://github.com/tsoding/koil), I was eager to attempt something similar in Rust.
I had to borrow some code from him due to my inexperience with JavaScript/TypeScript. (Thanks!)

# Building

```
  cargo build --target wasm32-unknown-unknown --release
  copy .\target\wasm32-unknown-unknown\release\minimal_wasm.wasm .\ #Windows
  python -m http.server 3000 #HTTP server to view the page at http://localhost:3000/
```

![example.png](/github/example.png)
