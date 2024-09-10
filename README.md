# Introduction
This is a simple proof of concept project for running Rust code in browser via Webassembly without the need for complicated librares.

## Inspiration
After [watching Tsoding's streams](https://www.youtube.com/@TsodingDaily) about his project [koil](https://github.com/tsoding/koil), I was eager to attempt something similar in Rust.
I had to borrow some code from him due to my inexperience with JavaScript/TypeScript. (Thanks!)

# Building

Run build.bat (build.sh if you're on Linux) and then start a simple HTTP server to view the page. (Using python is an easy way to do it)
```
  .\build.bat
  python -m http.server 3000
```
The script tries to use the `wasm2wat` utility to convert the .wasm file to a .wat file for debugging purposes, you can remove it from the build script if `wasm2wat` is not present in your system.

![example.png](/github/example.png)
