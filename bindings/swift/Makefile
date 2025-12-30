# Romcal Swift Binding Makefile
# Provides common tasks similar to npm scripts and taskipy

.PHONY: all build build-rust generate-bindings generate-types test test-verbose clean format lint help example

# Default target
all: build

# Help target
help:
	@echo "Romcal Swift Binding - Available targets:"
	@echo ""
	@echo "  make build           - Build the Swift package (requires Rust build first)"
	@echo "  make build-rust      - Build the Rust UniFFI library (release mode)"
	@echo "  make generate-bindings - Generate Swift bindings from UniFFI"
	@echo "  make generate-types  - Generate Swift types with Typeshare"
	@echo "  make test            - Run tests"
	@echo "  make test-verbose    - Run tests with verbose output"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make format          - Format Swift code with swift-format"
	@echo "  make lint            - Lint Swift code with SwiftLint"
	@echo "  make example         - Run the basic usage example"
	@echo "  make all             - Build everything (default)"
	@echo ""

# Build Rust UniFFI library
build-rust:
	@echo "Building Rust UniFFI library..."
	cd ../.. && cargo build --release -p romcal-uniffi

# Generate Swift bindings from UniFFI
generate-bindings: build-rust
	@echo "Generating Swift bindings..."
	cd ../.. && cargo run --release --bin uniffi-bindgen -- generate \
		--library target/release/libromcal_uniffi.dylib \
		--language swift \
		--out-dir bindings/swift/Sources/RomcalFFI
	@echo "Patching Swift bindings for CRomcalFFI import..."
	@sed -i '' 's/#if canImport(romcal_uniffiFFI)/#if canImport(CRomcalFFI)\nimport CRomcalFFI\n#elseif canImport(romcal_uniffiFFI)/' Sources/RomcalFFI/romcal_uniffi.swift

# Generate Swift types with Typeshare
# TypesCompat.swift provides BTreeMap, AnyCodable, and type aliases for untagged enums
generate-types:
	@echo "Generating Swift types with Typeshare..."
	cd ../.. && typeshare --config-file typeshare.toml ./core/src --lang=swift --output-file=bindings/swift/Sources/Romcal/Types.swift
	@echo "Formatting generated Types.swift..."
	swift format format --in-place Sources/Romcal/Types.swift
	@echo "Swift types generated and formatted"

# Build Swift package
build: generate-bindings generate-types
	@echo "Building Swift package..."
	swift build

# Build Swift package in release mode
build-release: generate-bindings generate-types
	@echo "Building Swift package (release)..."
	swift build -c release

# Run tests
test: build
	@echo "Running tests..."
	swift test

# Run tests with verbose output
test-verbose: build
	@echo "Running tests (verbose)..."
	swift test --verbose

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	swift package clean
	rm -rf .build
	rm -f Sources/RomcalFFI/romcal_uniffi.swift
	rm -f Sources/RomcalFFI/romcal_uniffiFFI.h
	rm -f Sources/RomcalFFI/romcal_uniffiFFI.modulemap

# Format Swift code (excludes generated files: RomcalFFI/, Types.swift)
format:
	@echo "Formatting Swift code..."
	swift format format --in-place \
		Sources/Romcal/Romcal.swift \
		Sources/Romcal/TypesCompat.swift \
		--recursive Tests Examples

# Lint Swift code (excludes generated files: RomcalFFI/, Types.swift)
lint:
	@echo "Linting Swift code..."
	swift format lint \
		Sources/Romcal/Romcal.swift \
		Sources/Romcal/TypesCompat.swift \
		--recursive Tests Examples

# Run the basic usage example
example: build
	@echo "Running basic usage example..."
	swift run BasicUsage
