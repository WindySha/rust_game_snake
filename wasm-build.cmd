
 cargo build --target wasm32-unknown-unknown --release --features web
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/snake.wasm
 cp index.html out
 cp -R assets out

