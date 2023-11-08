
 cargo build --target wasm32-unknown-unknown --release --features web
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/snake.wasm
 python3 -m http.server