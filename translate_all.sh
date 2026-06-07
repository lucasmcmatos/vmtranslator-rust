#!/bin/bash
set -e

cargo build --release 2>&1

for vm_file in tests/*.vm; do
    ./target/release/vmtranslator-rust "$vm_file"
done
