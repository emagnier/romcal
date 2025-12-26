#!/bin/bash

# Script to run quality checks on all adapters
# Usage: ./scripts/check-adapters.sh [adapter_name]

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔍 Running adapter quality checks..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Function to check a specific adapter
check_adapter() {
    local adapter_name=$1
    local adapter_path="$PROJECT_ROOT/bindings/$adapter_name"

    if [ ! -d "$adapter_path" ]; then
        echo "❌ Adapter '$adapter_name' not found at $adapter_path"
        return 1
    fi

    echo "🔍 Checking $adapter_name adapter..."
    cd "$adapter_path"

    # Run clippy
    echo "  🔍 Running clippy..."
    cargo clippy --release -- -D warnings

    # Run rustfmt
    echo "  🎨 Checking formatting..."
    cargo fmt -- --check

    # Run tests
    echo "  🧪 Running tests..."
    cargo test --release

    echo "✅ $adapter_name adapter checks completed!"
    echo ""
}

# Check if specific adapter was requested
if [ $# -eq 1 ]; then
    check_adapter "$1"
else
    # Check all available adapters
    echo "🔍 Discovering available adapters..."

    # Check WASM adapter
    if [ -d "$PROJECT_ROOT/bindings/wasm" ]; then
        check_adapter "wasm"
    fi

    # Check other adapters (when they exist)
    for adapter_dir in "$PROJECT_ROOT/bindings"/*; do
        if [ -d "$adapter_dir" ] && [ "$(basename "$adapter_dir")" != "wasm" ]; then
            adapter_name=$(basename "$adapter_dir")
            check_adapter "$adapter_name"
        fi
    done
fi

echo "🎉 All adapter quality checks completed successfully!"
echo ""
echo "💡 Available adapters:"
echo "   - wasm: WebAssembly bindings"
echo "   - python: Python bindings (planned)"
echo ""
echo "To check a specific adapter:"
echo "  ./scripts/check-adapters.sh wasm"
echo "  ./scripts/check-adapters.sh python  # when available"
