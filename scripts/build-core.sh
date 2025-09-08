#!/bin/bash

# Script to build and check the core module
# Usage: ./scripts/build-core.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🚀 Starting core build and lint..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core directory
cd "$PROJECT_ROOT/core"

# Build the core module
echo "🔨 Building core module..."
cargo build --release

echo ""
echo "✅ Core build completed!"

# Run clippy linter
echo ""
echo "🔍 Running clippy linter..."
cargo clippy --release -- -D warnings

echo ""
echo "✅ Clippy linting completed!"

# Run rustfmt check
echo ""
echo "🎨 Checking code formatting with rustfmt..."
cargo fmt -- --check

echo ""
echo "✅ Code formatting check completed!"

echo ""
echo "🎉 All core build and lint tasks completed successfully!"
