#!/bin/bash

# Script to verify that generated types are in sync with Rust types
# Usage: ./scripts/check-types-sync.sh
#
# This script regenerates types for all bindings and checks if they match
# what's committed in the repository. If there are differences, it means
# someone modified Rust types without regenerating the binding types.
#
# Note: Swift and Kotlin require typeshare-cli which may not be available
# in all environments. These checks are optional and will be skipped if
# typeshare is not installed.

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

# Regenerate Swift types (optional - requires typeshare-cli)
if command -v typeshare &> /dev/null; then
    echo "🔄 Regenerating Swift types..."
    cd "$PROJECT_ROOT/bindings/swift"
    make generate-types
    FILES_TO_CHECK="$FILES_TO_CHECK bindings/swift/Sources/Romcal/Types.swift"

    echo "🔄 Regenerating Kotlin types..."
    cd "$PROJECT_ROOT/bindings/kotlin"
    make generate-types
    FILES_TO_CHECK="$FILES_TO_CHECK bindings/kotlin/src/main/kotlin/com/romcal/types/Types.kt"
else
    echo "⚠️  Skipping Swift/Kotlin types (typeshare-cli not installed)"
fi

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
    if command -v typeshare &> /dev/null; then
        echo "  # Swift"
        echo "  cd bindings/swift && make generate-types"
        echo ""
        echo "  # Kotlin"
        echo "  cd bindings/kotlin && make generate-types"
        echo ""
    fi
    exit 1
fi

echo "✅ Generated types are in sync with Rust types!"
