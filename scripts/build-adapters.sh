#!/bin/bash

# Build script for all romcal adapters
# Usage: ./scripts/build-adapters.sh [adapter_name]

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔨 Building Romcal adapters..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Function to build a specific adapter
build_adapter() {
    local adapter_name=$1
    local adapter_path="$PROJECT_ROOT/core/adapters/$adapter_name"

    if [ ! -d "$adapter_path" ]; then
        echo "❌ Adapter '$adapter_name' not found at $adapter_path"
        return 1
    fi

    echo "🔨 Building $adapter_name adapter..."
    cd "$adapter_path"

    # Build the adapter
    cargo build --release

    echo "✅ $adapter_name adapter built successfully!"
    echo ""
}

# Function to build WASM adapter with special handling
build_wasm_adapter() {
    echo "🔨 Building WASM adapter..."
    cd "$PROJECT_ROOT/core/adapters/wasm"

    # Build with wasm-pack
    wasm-pack build --target nodejs --out-dir ../../../bindings/wasm/pkg

    echo "✅ WASM adapter built successfully!"
    echo "📦 Output location: bindings/wasm/pkg/"
    echo ""
}

# Check if specific adapter was requested
if [ $# -eq 1 ]; then
    if [ "$1" = "wasm" ]; then
        build_wasm_adapter
    else
        build_adapter "$1"
    fi
else
    # Build all available adapters
    echo "🔍 Discovering available adapters..."

    # Build WASM adapter
    if [ -d "$PROJECT_ROOT/core/adapters/wasm" ]; then
        build_wasm_adapter
    fi

    # Build other adapters (when they exist)
    for adapter_dir in "$PROJECT_ROOT/core/adapters"/*; do
        if [ -d "$adapter_dir" ] && [ "$(basename "$adapter_dir")" != "wasm" ]; then
            adapter_name=$(basename "$adapter_dir")
            build_adapter "$adapter_name"
        fi
    done
fi

echo "🎉 All adapters build completed successfully!"
echo ""
echo "💡 Available adapters:"
echo "   - wasm: WebAssembly bindings"
echo "   - python: Python bindings (planned)"
echo "   - dart: Dart/Flutter bindings (planned)"
echo ""
echo "To build a specific adapter:"
echo "  ./scripts/build-adapters.sh wasm"
echo "  ./scripts/build-adapters.sh python  # when available"
