#!/bin/bash
set -e

BIN=./target/release/vmtranslator-rust

echo "=== Part 2 Tests ==="
echo ""

echo "[1/6] ProgramFlow/BasicLoop (single file)"
$BIN tests/part2/ProgramFlow/BasicLoop/BasicLoop.vm

echo "[2/6] ProgramFlow/FibonacciSeries (single file)"
$BIN tests/part2/ProgramFlow/FibonacciSeries/FibonacciSeries.vm

echo "[3/6] FunctionCalls/SimpleFunction (single file)"
$BIN tests/part2/FunctionCalls/SimpleFunction/SimpleFunction.vm

echo "[4/6] FunctionCalls/NestedCall (directory)"
$BIN tests/part2/FunctionCalls/NestedCall/

echo "[5/6] FunctionCalls/FibonacciElement (directory)"
$BIN tests/part2/FunctionCalls/FibonacciElement/

echo "[6/6] FunctionCalls/StaticsTest (directory)"
$BIN tests/part2/FunctionCalls/StaticsTest/

echo ""
echo "All .asm files generated. Open the CPU Emulator and load the .tst files to validate."
