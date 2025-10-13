#!/bin/bash

# Script to generate JSON schemas from Rust types
# Usage: ./scripts/generate-schema.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🚀 Starting schema generation..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the core directory
cd "$PROJECT_ROOT/core"

# Build the schema generation binary
echo "🔨 Building schema generation binary..."
cargo build --release --bin generate-schema --features schema-gen

# Run the schema generation
echo ""
echo "🔧 Generating JSON schemas..."
cargo run --release --bin generate-schema --features schema-gen

echo ""
echo "✅ Schema generation completed!"
