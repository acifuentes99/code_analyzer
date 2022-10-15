#!/bin/bash
cargo build --release
cp target/release/rust ./code_analyzer
