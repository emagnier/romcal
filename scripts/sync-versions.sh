#!/bin/bash

# Script to synchronize version across all packages
# Usage: ./scripts/sync-versions.sh [--check]
#
# Without arguments: Updates all package files to match VERSION
# With --check: Only verifies versions are in sync (for CI)

set -e

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION_FILE="$PROJECT_ROOT/VERSION"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if VERSION file exists
if [ ! -f "$VERSION_FILE" ]; then
    echo -e "${RED}ERROR: VERSION file not found at $VERSION_FILE${NC}"
    exit 1
fi

# Read version from VERSION file (trim whitespace)
VERSION=$(cat "$VERSION_FILE" | tr -d '[:space:]')

if [ -z "$VERSION" ]; then
    echo -e "${RED}ERROR: VERSION file is empty${NC}"
    exit 1
fi

echo "📦 Version: $VERSION"
echo ""

# Mode: check or update
CHECK_MODE=false
if [ "$1" = "--check" ]; then
    CHECK_MODE=true
    echo "🔍 Running in check mode..."
else
    echo "🔄 Running in update mode..."
fi
echo ""

# Track errors
ERRORS=0

# Function to check/update Cargo.toml version
update_cargo_version() {
    local file="$1"
    local relative_path="${file#$PROJECT_ROOT/}"

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠ File not found: $relative_path${NC}"
        return
    fi

    # Extract current version from Cargo.toml
    local current=$(grep -m1 '^version = ' "$file" | sed 's/version = "\(.*\)"/\1/')

    if [ "$current" = "$VERSION" ]; then
        echo -e "${GREEN}✓ $relative_path${NC}"
    elif [ "$CHECK_MODE" = true ]; then
        echo -e "${RED}✗ $relative_path (found: $current, expected: $VERSION)${NC}"
        ERRORS=$((ERRORS + 1))
    else
        # Update the version
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
        else
            sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
        fi
        echo -e "${GREEN}✓ $relative_path (updated from $current)${NC}"
    fi
}

# Function to update romcal dependency version in cli/Cargo.toml
update_cli_dependency() {
    local file="$PROJECT_ROOT/cli/Cargo.toml"
    local relative_path="cli/Cargo.toml"

    if [ ! -f "$file" ]; then
        return
    fi

    # Extract current romcal dependency version
    local current=$(grep 'romcal = { version = ' "$file" | sed 's/.*version = "\([^"]*\)".*/\1/')

    if [ -z "$current" ]; then
        return
    fi

    if [ "$current" = "$VERSION" ]; then
        echo -e "${GREEN}✓ $relative_path (romcal dep)${NC}"
    elif [ "$CHECK_MODE" = true ]; then
        echo -e "${RED}✗ $relative_path romcal dependency (found: $current, expected: $VERSION)${NC}"
        ERRORS=$((ERRORS + 1))
    else
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/romcal = { version = \"[^\"]*\"/romcal = { version = \"$VERSION\"/" "$file"
        else
            sed -i "s/romcal = { version = \"[^\"]*\"/romcal = { version = \"$VERSION\"/" "$file"
        fi
        echo -e "${GREEN}✓ $relative_path (romcal dep updated from $current)${NC}"
    fi
}

# Function to update romcal peerDependency version in unplugin/package.json
update_unplugin_peer_dependency() {
    local file="$PROJECT_ROOT/bindings/unplugin/package.json"
    local relative_path="bindings/unplugin/package.json"

    if [ ! -f "$file" ]; then
        return
    fi

    # Extract current romcal peerDependency version (only from peerDependencies section)
    # Use grep -A1 to get the line after "peerDependencies", then extract the version
    local current=$(grep -A1 '"peerDependencies"' "$file" | grep '"romcal":' | sed 's/.*"romcal": "\^\([^"]*\)".*/\1/')

    if [ -z "$current" ]; then
        return
    fi

    if [ "$current" = "$VERSION" ]; then
        echo -e "${GREEN}✓ $relative_path (romcal peerDep)${NC}"
    elif [ "$CHECK_MODE" = true ]; then
        echo -e "${RED}✗ $relative_path romcal peerDependency (found: ^$current, expected: ^$VERSION)${NC}"
        ERRORS=$((ERRORS + 1))
    else
        # Use node to update the peerDependency (most reliable for JSON)
        node -e "
            const fs = require('fs');
            const pkg = JSON.parse(fs.readFileSync('$file', 'utf8'));
            pkg.peerDependencies.romcal = '^$VERSION';
            fs.writeFileSync('$file', JSON.stringify(pkg, null, 2) + '\n');
        "
        echo -e "${GREEN}✓ $relative_path (romcal peerDep updated from ^$current)${NC}"
    fi
}

# Function to check/update package.json version
update_package_json() {
    local file="$1"
    local relative_path="${file#$PROJECT_ROOT/}"

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠ File not found: $relative_path${NC}"
        return
    fi

    # Extract current version from package.json
    local current=$(grep -m1 '"version":' "$file" | sed 's/.*"version": "\([^"]*\)".*/\1/')

    if [ "$current" = "$VERSION" ]; then
        echo -e "${GREEN}✓ $relative_path${NC}"
    elif [ "$CHECK_MODE" = true ]; then
        echo -e "${RED}✗ $relative_path (found: $current, expected: $VERSION)${NC}"
        ERRORS=$((ERRORS + 1))
    else
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$file"
        else
            sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$file"
        fi
        echo -e "${GREEN}✓ $relative_path (updated from $current)${NC}"
    fi
}

# Function to check/update pyproject.toml version
update_pyproject() {
    local file="$1"
    local relative_path="${file#$PROJECT_ROOT/}"

    if [ ! -f "$file" ]; then
        echo -e "${YELLOW}⚠ File not found: $relative_path${NC}"
        return
    fi

    # Extract current version from pyproject.toml
    local current=$(grep -m1 '^version = ' "$file" | sed 's/version = "\(.*\)"/\1/')

    if [ "$current" = "$VERSION" ]; then
        echo -e "${GREEN}✓ $relative_path${NC}"
    elif [ "$CHECK_MODE" = true ]; then
        echo -e "${RED}✗ $relative_path (found: $current, expected: $VERSION)${NC}"
        ERRORS=$((ERRORS + 1))
    else
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
        else
            sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$file"
        fi
        echo -e "${GREEN}✓ $relative_path (updated from $current)${NC}"
    fi
}

# Update all Cargo.toml files
echo "Rust packages:"
update_cargo_version "$PROJECT_ROOT/core/Cargo.toml"
update_cargo_version "$PROJECT_ROOT/cli/Cargo.toml"
update_cli_dependency
update_cargo_version "$PROJECT_ROOT/bindings/wasm/Cargo.toml"
update_cargo_version "$PROJECT_ROOT/bindings/uniffi/Cargo.toml"
update_cargo_version "$PROJECT_ROOT/tools/Cargo.toml"

echo ""
echo "TypeScript packages:"
update_package_json "$PROJECT_ROOT/bindings/typescript/package.json"
update_package_json "$PROJECT_ROOT/bindings/unplugin/package.json"
update_unplugin_peer_dependency

echo ""
echo "Python package:"
update_pyproject "$PROJECT_ROOT/bindings/python/pyproject.toml"

echo ""

# Summary
if [ "$CHECK_MODE" = true ]; then
    if [ $ERRORS -gt 0 ]; then
        echo -e "${RED}❌ Version sync check failed: $ERRORS file(s) out of sync${NC}"
        echo ""
        echo "Run './scripts/sync-versions.sh' to fix."
        exit 1
    else
        echo -e "${GREEN}✅ All versions are in sync!${NC}"
    fi
else
    echo -e "${GREEN}✅ All versions synchronized to $VERSION${NC}"
fi
