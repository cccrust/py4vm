#!/bin/bash

echo "=== py4vm Test Runner ==="
echo ""

cd /Users/Shared/ccc/project/py4vm
cargo build 2>/dev/null

echo "Running tests..."
echo ""

for test in hello add loop ifelse arith import from_import mix; do
    echo "=== $test ==="
    timeout 2 ./target/debug/py4vm "$test" 2>&1 || echo "(error or timeout)"
    echo ""
done

echo "=== Done ==="