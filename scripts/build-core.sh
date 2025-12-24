#!/bin/bash

# Script to build the core romcal module (without adapters)
# Usage: ./scripts/build-core.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔨 Building core module..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core romcal directory
cd "$PROJECT_ROOT/core"

# Build only the romcal module (not the adapters)
echo "🔨 Building core romcal..."
cargo build --package romcal --release

echo ""
echo "✅ Core romcal build completed successfully!"
echo ""
echo "💡 Note: This script only builds the romcal module."
echo "   To run quality checks, use: ./scripts/check-core.sh"
echo "   To build adapters, use: ./scripts/build-adapters.sh"
