#!/bin/bash

# CI/CD pipeline script for the entire romcal project
# Usage: ./scripts/ci.sh

set -euo pipefail

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Store original directory
ORIGINAL_DIR=$(pwd)

# Utility functions
run_step() {
    local step_name="$1"
    local command="$2"
    local start_time=$(date +%s)

    echo "🔄 Running: $step_name"
    if eval "$command"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "✅ $step_name completed successfully (${duration}s)"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo "❌ Failed: $step_name (${duration}s)"
        return 1
    fi
}

run_script() {
    local script_name="$1"
    local step_name="$2"
    run_step "$step_name" "\"$PROJECT_ROOT/scripts/$script_name\""
}

cleanup() {
    cd "$ORIGINAL_DIR"
}

# Set up cleanup trap
trap cleanup EXIT

# Start timing
PIPELINE_START_TIME=$(date +%s)

echo "🚀 Starting CI/CD pipeline..."
echo "📁 Project root: $PROJECT_ROOT"
echo "🕐 Started at: $(date)"
echo ""

# Step 1: Build core module
run_script "build-core.sh" "1️⃣ Building core module"

# Step 2: Check core quality
run_script "check-core.sh" "2️⃣ Checking core quality"

# Step 3: Build all adapters
run_script "build-adapters.sh" "3️⃣ Building all adapters"

# Step 4: Check adapter quality
run_script "check-adapters.sh" "4️⃣ Checking adapter quality"

# Step 5: Generate schemas
run_script "generate-schema.sh" "5️⃣ Generating JSON schemas"

# Step 6: Build tools
if [ -d "$PROJECT_ROOT/tools" ]; then
    run_step "6️⃣ Building and checking tools" "cd '$PROJECT_ROOT/tools' && cargo build --release && cargo clippy --release -- -D warnings && cargo fmt -- --check && cargo test --release"
fi

# Step 7: Validate calendars
run_script "validate-calendars.sh" "7️⃣ Validating calendar files"

# Step 8: Validate resources
run_script "validate-resources.sh" "8️⃣ Validating resource files"

# Step 9: Integration tests (if they exist)
if [ -d "$PROJECT_ROOT/tests" ]; then
    run_step "9️⃣ Running integration tests" "cd '$PROJECT_ROOT' && cargo test --release"
fi

# Calculate total duration
PIPELINE_END_TIME=$(date +%s)
TOTAL_DURATION=$((PIPELINE_END_TIME - PIPELINE_START_TIME))
HOURS=$((TOTAL_DURATION / 3600))
MINUTES=$(((TOTAL_DURATION % 3600) / 60))
SECONDS=$((TOTAL_DURATION % 60))

echo "🎉 CI/CD pipeline completed successfully!"
echo ""
echo "📊 Summary:"
echo "   ✅ Core module built and checked"
echo "   ✅ All adapters built and checked"
echo "   ✅ JSON schemas generated"
echo "   ✅ Calendar files validated"
echo "   ✅ Resource files validated"
echo "   ✅ Tools built and checked"
echo "   ✅ All quality checks passed"
echo ""
echo "⏱️ Total duration: ${HOURS}h ${MINUTES}m ${SECONDS}s"
echo "🕐 Completed at: $(date)"
echo ""
echo "🚀 Ready for deployment!"
