#!/bin/bash

# JSON Round-trip Test Script for Romcal
# This script runs the JSON round-trip test to verify data integrity

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CORE_DIR="$PROJECT_ROOT/core"

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print help
print_help() {
    echo "JSON Round-trip Test Script for Romcal"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  all                       Test all JSON files (calendars and resources)"
    echo "  calendars                 Test only calendar definition files"
    echo "  resources                 Test only resource files"
    echo "  <file_path>               Test a specific JSON file"
    echo "  --help, -h                Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 all"
    echo "  $0 calendars"
    echo "  $0 resources"
    echo "  $0 data/definitions/general_roman/general_roman.json"
    echo ""
    echo "The script will:"
    echo "  1. Build the json_roundtrip_test binary"
    echo "  2. Run the specified test command"
    echo "  3. Display results with colored output"
}

# Function to build the test binary
build_test_binary() {
    print_status $BLUE "🔨 Building json_roundtrip_test binary..."
    cd "$CORE_DIR"

    if cargo build --release --bin json_roundtrip_test; then
        print_status $GREEN "✅ Binary built successfully!"
    else
        print_status $RED "❌ Failed to build binary"
        exit 1
    fi
}

# Function to run the test
run_test() {
    local command=$1
    local binary_path="$PROJECT_ROOT/target/release/json_roundtrip_test"

    print_status $BLUE "🧪 Running JSON round-trip test: $command"
    echo ""

    if [ -f "$binary_path" ]; then
        # Change to project root directory to ensure relative paths work
        cd "$PROJECT_ROOT"
        "$binary_path" "$command"
    else
        print_status $RED "❌ Binary not found at $binary_path"
        print_status $YELLOW "💡 Try running: $0 build"
        exit 1
    fi
}

# Main script logic
main() {
    # Check if we're in the right directory
    if [ ! -f "$CORE_DIR/Cargo.toml" ]; then
        print_status $RED "❌ Error: Cannot find core/Cargo.toml"
        print_status $YELLOW "💡 Please run this script from the project root or scripts directory"
        exit 1
    fi

    # Parse arguments
    case "${1:-}" in
        ""|"--help"|"-h")
            print_help
            exit 0
            ;;
        "build")
            build_test_binary
            exit 0
            ;;
        "all"|"calendars"|"resources")
            build_test_binary
            run_test "$1"
            ;;
        *)
            # Assume it's a file path
            build_test_binary
            run_test "$1"
            ;;
    esac
}

# Run main function with all arguments
main "$@"
