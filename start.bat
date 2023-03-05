@echo off
cd %~dp0
cargo build --target wasm32-unknown-unknown
