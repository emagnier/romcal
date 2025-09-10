#!/bin/bash

# Script to build the core module (without adapters)
# Usage: ./scripts/build-core.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔨 Building core module..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core directory
cd "$PROJECT_ROOT/core"

# Build only the core module (not the adapters)
echo "🔨 Building core module..."
cargo build --package romcal-core --release

echo ""
echo "✅ Core build completed successfully!"
echo ""
echo "💡 Note: This script only builds the core module."
echo "   To run quality checks, use: ./scripts/check-core.sh"
echo "   To build adapters, use: ./scripts/build-adapters.sh"
