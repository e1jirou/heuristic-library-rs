#!/bin/bash

cargo build
cd tools
cargo run --release --bin tester ../target/debug/ahc < $1 > out.txt
cd ..
cargo clean
