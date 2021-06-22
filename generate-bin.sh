#!/usr/bin/sh

cargo build --release
cp ./target/release/dwm-status ./bin/dwm-status-$1-$2-$3

