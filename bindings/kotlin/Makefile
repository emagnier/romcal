# Romcal Kotlin Binding Makefile
# Provides common tasks similar to npm scripts and taskipy

.PHONY: all build generate-types test clean help

# Default target
all: build

# Help target
help:
	@echo "Romcal Kotlin Binding - Available targets:"
	@echo ""
	@echo "  make build           - Build the Kotlin package"
	@echo "  make generate-types  - Generate Kotlin types with Typeshare"
	@echo "  make test            - Run tests"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make all             - Build everything (default)"
	@echo ""

# Generate Kotlin types with Typeshare
generate-types:
	@echo "Generating Kotlin types with Typeshare..."
	cd ../.. && typeshare --config-file typeshare.toml ./core/src --lang=kotlin --output-file=bindings/kotlin/src/main/kotlin/com/romcal/types/Types.kt
	@echo "Kotlin types generated (TypesCompat.kt provides compatibility types)"

# Build Kotlin package
build: generate-types
	@echo "Building Kotlin package..."
	./gradlew build

# Run tests
test: build
	@echo "Running tests..."
	./gradlew test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	./gradlew clean
