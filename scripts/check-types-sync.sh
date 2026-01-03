#!/bin/bash

# Script to verify that generated types are in sync with Rust types
# Usage: ./scripts/check-types-sync.sh
#
# This script regenerates types for all bindings and checks if they match
# what's committed in the repository. If there are differences, it means
# someone modified Rust types without regenerating the binding types.

set -euo pipefail

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Store original directory
ORIGINAL_DIR=$(pwd)

cleanup() {
    cd "$ORIGINAL_DIR"
}

trap cleanup EXIT

echo "🔍 Checking types synchronization..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

FILES_TO_CHECK=""

# Regenerate TypeScript types
echo "🔄 Regenerating TypeScript types..."
cd "$PROJECT_ROOT/bindings/typescript"
npm run generate-types --silent
FILES_TO_CHECK="$FILES_TO_CHECK bindings/typescript/src/types/"

# Regenerate Python types
echo "🔄 Regenerating Python types..."
cd "$PROJECT_ROOT/bindings/python"
uv run task generate-types
FILES_TO_CHECK="$FILES_TO_CHECK bindings/python/src/romcal/types.py"

cd "$PROJECT_ROOT"

echo ""
echo "🔍 Checking for differences..."

# Check if any generated files have changed
if ! git diff --exit-code $FILES_TO_CHECK 2>/dev/null; then
    echo ""
    echo "❌ ERROR: Generated types are out of sync with Rust types!"
    echo ""
    echo "To fix this, regenerate the types and commit the changes:"
    echo ""
    echo "  # TypeScript"
    echo "  cd bindings/typescript && npm run generate-types"
    echo ""
    echo "  # Python"
    echo "  cd bindings/python && uv run task generate-types"
    echo ""
    exit 1
fi

echo "✅ Generated types are in sync with Rust types!"
