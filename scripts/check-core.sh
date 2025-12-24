#!/bin/bash

# Script to run quality checks on the core module
# Usage: ./scripts/check-core.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔍 Running core romcal quality checks..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core directory
cd "$PROJECT_ROOT/core"

# Run clippy linter on core
echo "🔍 Running clippy linter on core romcal..."
cargo clippy --package romcal --release -- -D warnings

echo ""
echo "✅ Clippy linting completed!"

# Run rustfmt check on core romcal
echo ""
echo "🎨 Checking code formatting with rustfmt..."
cargo fmt --package romcal -- --check

echo ""
echo "✅ Code formatting check completed!"

# Run tests on core romcal
echo ""
echo "🧪 Running tests on core romcal..."
cargo test --package romcal --release

echo ""
echo "✅ Tests completed!"

echo ""
echo "🎉 All core romcal quality checks completed successfully!"
echo ""
echo "💡 Note: This script only checks the core romcal module."
echo "   To check adapters, use: ./scripts/check-adapters.sh"
echo "   To run all checks, use: ./scripts/check-all.sh"
