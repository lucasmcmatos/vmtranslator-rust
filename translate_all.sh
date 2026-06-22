#!/bin/bash
set -e

cargo build --release 2>&1

echo "=== Part 1: single .vm files ==="
for vm_file in tests/part1/*/*.vm; do
    echo "Translating $vm_file"
    ./target/release/vmtranslator-rust "$vm_file"
done
