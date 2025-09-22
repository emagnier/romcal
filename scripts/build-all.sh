#!/bin/bash

# Build script for the entire romcal project
# Usage: ./scripts/build-all.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🚀 Building entire Romcal project..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Build core module
echo "1️⃣ Building core module..."
cd "$PROJECT_ROOT"
./scripts/build-core.sh

echo ""

# Build all adapters
echo "2️⃣ Building all adapters..."
./scripts/build-adapters.sh

echo ""

# Build CLI
echo "3️⃣ Building CLI..."
./scripts/build-cli.sh

echo ""

# Build tools (if they exist)
if [ -d "$PROJECT_ROOT/tools" ]; then
    echo "4️⃣ Building tools..."
    cd "$PROJECT_ROOT/tools"
    cargo build --release
    echo "✅ Tools built successfully!"
    echo ""
fi

echo "🎉 Entire project build completed successfully!"
echo ""
echo "📦 Build outputs:"
echo "   - Core library: target/release/libromcal_core.rlib"
echo "   - Core tools: target/release/generate-schema"
echo "   - WASM: bindings/typescript/pkg/"
echo "   - CLI: target/release/romcal"
echo "   - Tools: target/release/validate-json"
echo ""
echo "To run tests:"
echo "  cargo test"
echo ""
echo "To run WASM tests:"
echo "  cd bindings/typescript && npm test"
