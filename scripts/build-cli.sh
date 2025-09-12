#!/bin/bash

# Script to build the CLI module
# Usage: ./scripts/build-cli.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔨 Building CLI module..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the CLI directory
cd "$PROJECT_ROOT/cli"

# Build the CLI module
echo "🔨 Building CLI module..."
cargo build --package romcal-cli --release

echo ""
echo "✅ CLI build completed successfully!"
echo ""
echo "📦 Build output:"
echo "   - CLI binary: target/release/romcal"
echo ""
echo "💡 Note: This script only builds the CLI module."
echo "   To run quality checks, use: ./scripts/check-cli.sh"
echo "   To build everything, use: ./scripts/build-all.sh"
echo ""
echo "🚀 To test the CLI:"
echo "   ./target/release/romcal --help"
