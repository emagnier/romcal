#!/bin/bash

# Script to run quality checks on the core module
# Usage: ./scripts/check-core.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔍 Running core quality checks..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core directory
cd "$PROJECT_ROOT/core"

# Run clippy linter on core
echo "🔍 Running clippy linter on core..."
cargo clippy --package romcal-core --release -- -D warnings

echo ""
echo "✅ Clippy linting completed!"

# Run rustfmt check on core
echo ""
echo "🎨 Checking code formatting with rustfmt..."
cargo fmt --package romcal-core -- --check

echo ""
echo "✅ Code formatting check completed!"

# Run tests on core
echo ""
echo "🧪 Running tests on core..."
cargo test --package romcal-core --release

echo ""
echo "✅ Tests completed!"

echo ""
echo "🎉 All core quality checks completed successfully!"
echo ""
echo "💡 Note: This script only checks the core module."
echo "   To check adapters, use: ./scripts/check-adapters.sh"
echo "   To run all checks, use: ./scripts/check-all.sh"
