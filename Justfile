lib:
    cargo rustc --target wasm32-wasi -p game --crate-type=cdylib
    wasm-tools component new ./target/wasm32-wasi/debug/game.wasm -o target/debug/game.wasm --adapt vendor/wasi_snapshot_preview1.reactor.wasm

watch:
    cargo watch -C game -w ../wit -w . -- just lib 

launcher:
    cargo run -p launcher

hotreload:
    cargo run -p hotreload-host