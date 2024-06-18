lib:
    cargo build --target wasm32-wasi -p game
    wasm-tools component new ./target/wasm32-wasi/debug/game.wasm -o target/debug/game.wasm --adapt vendor/wasi_snapshot_preview1.reactor.wasm

run:
    cargo run -p hotreload-host