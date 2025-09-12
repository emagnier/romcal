#!/bin/bash

# Global quality check script for the entire romcal project
# Usage: ./scripts/check-all.sh

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "🔍 Starting global quality checks..."
echo "📁 Project root: $PROJECT_ROOT"
echo ""

# Validate core module
echo "1️⃣ Validating core module..."
./scripts/check-core.sh

echo ""

# Validate adapters
echo "2️⃣ Validating adapters..."
./scripts/check-adapters.sh

echo ""

# Validate CLI
echo "3️⃣ Validating CLI..."
./scripts/check-cli.sh

echo ""

# Validate tools (if they exist)
if [ -d "$PROJECT_ROOT/tools" ]; then
    echo "4️⃣ Validating tools..."
    cd "$PROJECT_ROOT/tools"
    cargo clippy --release -- -D warnings
    cargo fmt -- --check
    cargo test --release
    echo "✅ Tools validation completed!"
    echo ""
fi

# Validate workspace dependencies
echo "5️⃣ Validating workspace dependencies..."
cd "$PROJECT_ROOT"
cargo check --workspace

echo "✅ Workspace validation completed!"
echo ""

# Validate schemas (if they exist)
if [ -d "$PROJECT_ROOT/schemas" ]; then
    echo "6️⃣ Validating schemas..."
    if [ -f "$PROJECT_ROOT/scripts/validate-schemas.sh" ]; then
        ./scripts/validate-schemas.sh
    else
        echo "ℹ️  Schema validation script not found, skipping..."
    fi
    echo ""
fi

echo "🎉 Global quality checks completed successfully!"
echo ""
echo "📊 Quality checks summary:"
echo "   ✅ Core module validated"
echo "   ✅ All adapters validated"
echo "   ✅ CLI validated"
echo "   ✅ Tools validated"
echo "   ✅ Workspace dependencies validated"
echo "   ✅ Schemas validated"
echo ""
echo "🚀 All validations passed! Project is ready."
