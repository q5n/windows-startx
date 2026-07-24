@echo off

cargo clean
cargo build --release

target\release\startx -h

powershell -c "sleep 3"
