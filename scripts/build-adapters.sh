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
    local adapter_path="$PROJECT_ROOT/bindings/$adapter_name"

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
    cd "$PROJECT_ROOT/bindings/wasm"

    # Build with wasm-pack using 'web' target for universal compatibility
    # The 'web' target generates an init() function that works in all environments
    wasm-pack build --target web --out-dir ../typescript/pkg

    echo "✅ WASM adapter built successfully!"
    echo "📦 Output location: bindings/typescript/pkg/"
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
    # Build all available Rust adapters
    echo "🔍 Discovering available adapters..."

    # Build WASM adapter first (special handling with wasm-pack)
    if [ -d "$PROJECT_ROOT/bindings/wasm" ]; then
        build_wasm_adapter
    fi

    # Build other Rust adapters (those with Cargo.toml, excluding wasm)
    for adapter_dir in "$PROJECT_ROOT/bindings"/*; do
        adapter_name=$(basename "$adapter_dir")
        if [ -d "$adapter_dir" ] && [ -f "$adapter_dir/Cargo.toml" ] && [ "$adapter_name" != "wasm" ]; then
            build_adapter "$adapter_name"
        fi
    done
fi

echo "🎉 All adapters build completed successfully!"
echo ""
echo "💡 Available adapters:"
echo "   - wasm: WebAssembly bindings"
echo "   - python: Python bindings (planned)"
echo ""
echo "To build a specific adapter:"
echo "  ./scripts/build-adapters.sh wasm"
echo "  ./scripts/build-adapters.sh python  # when available"
