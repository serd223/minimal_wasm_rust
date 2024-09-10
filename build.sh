cargo build --target wasm32-unknown-unknown --release && cp ./target/wasm32-unknown-unknown/release/minimal_wasm.wasm ./ && wasm2wat minimal_wasm.wasm -o minimal_wasm.wat
