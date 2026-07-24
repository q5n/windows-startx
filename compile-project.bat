@echo off

cargo clean
cargo build --release


powershell -c "sleep 3"
