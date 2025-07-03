#!/bin/bash

# Run tests
cargo test

# Build and run the release version
cargo build --release

# Copy the binary to /usr/local/bin
sudo cp target/release/n /usr/local/bin