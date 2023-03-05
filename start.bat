@echo off
cd %~dp0
cargo install wasm-pack && wasm-pack build --target web
