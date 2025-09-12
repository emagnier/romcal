#!/bin/bash

# Script to run quality checks on the CLI module
# Usage: ./scripts/check-cli.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔍 Running CLI quality checks..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the CLI directory
cd "$PROJECT_ROOT/cli"

# Run clippy linter on CLI
echo "🔍 Running clippy linter on CLI..."
cargo clippy --package romcal-cli --release -- -D warnings

echo ""
echo "✅ Clippy linting completed!"

# Run rustfmt check on CLI
echo ""
echo "🎨 Checking code formatting with rustfmt..."
cargo fmt --package romcal-cli -- --check

echo ""
echo "✅ Code formatting check completed!"

# Run tests on CLI
echo ""
echo "🧪 Running tests on CLI..."
cargo test --package romcal-cli --release

echo ""
echo "✅ Tests completed!"

echo ""
echo "🎉 All CLI quality checks completed successfully!"
echo ""
echo "💡 Note: This script only checks the CLI module."
echo "   To check everything, use: ./scripts/check-all.sh"
echo ""
echo "🚀 To test the CLI manually:"
echo "   cargo run -- --help"
echo "   cargo run -- list-locales"
echo "   cargo run -- config"
