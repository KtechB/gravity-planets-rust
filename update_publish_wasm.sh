
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/debug/gravity-planets.wasm gravity-planets.wasm