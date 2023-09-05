#!/bin/sh
cargo build --target x86_64-pc-windows-gnu && ./target/x86_64-pc-windows-gnu/debug/leds.exe --color 0xff00ff
