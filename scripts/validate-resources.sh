#!/bin/bash

# Script to validate all resource JSON files against the schema
# Usage: ./scripts/validate-resources.sh [project_root]

set -e

# Get the project root directory
if [ $# -eq 1 ]; then
    PROJECT_ROOT="$1"
else
    # Default to parent directory of scripts/
    PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fi

echo "🚀 Starting resource validation..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Change to the tools directory
cd "$PROJECT_ROOT/tools"

# Build the validation binary
echo "🔨 Building validation binary..."
cargo build --release --bin validate-json

# Run the validation
echo ""
echo "🔍 Running validation..."
cargo run --release --bin validate-json -- "data/resources/**/*.json" "schemas/resources.json" "$PROJECT_ROOT"
