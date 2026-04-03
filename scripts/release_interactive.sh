#!/bin/bash

cargo build --release
cd tools
cargo run --release --bin tester ../target/release/ahc < $1 > out.txt
cd ..
cargo clean
