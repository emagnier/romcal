# Romcal Kotlin Binding Makefile
# Provides common tasks for building the Kotlin binding with UniFFI

.PHONY: all build build-rust generate-bindings generate-types copy-native-lib test clean help

# Detect OS for library extension and resource directory
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
    LIB_EXT := dylib
    ifeq ($(UNAME_M),arm64)
        RESOURCE_DIR := darwin-aarch64
    else
        RESOURCE_DIR := darwin-x86-64
    endif
else ifeq ($(UNAME_S),Linux)
    LIB_EXT := so
    RESOURCE_DIR := linux-x86-64
else
    LIB_EXT := dll
    RESOURCE_DIR := win32-x86-64
endif

# Default target
all: build

# Help target
help:
	@echo "Romcal Kotlin Binding - Available targets:"
	@echo ""
	@echo "  make build             - Build everything (Rust + bindings + Kotlin)"
	@echo "  make build-rust        - Build the Rust UniFFI library (release mode)"
	@echo "  make generate-bindings - Generate Kotlin bindings from UniFFI"
	@echo "  make generate-types    - Generate Kotlin types with Typeshare"
	@echo "  make copy-native-lib   - Copy native library to resources"
	@echo "  make test              - Run tests"
	@echo "  make clean             - Clean build artifacts"
	@echo "  make all               - Build everything (default)"
	@echo ""

# Build Rust UniFFI library
build-rust:
	@echo "Building Rust UniFFI library..."
	cd ../.. && cargo build --release -p romcal-uniffi

# Generate Kotlin bindings from UniFFI
generate-bindings: build-rust
	@echo "Generating Kotlin bindings..."
	mkdir -p src/main/kotlin/com/romcal/ffi
	cd ../.. && cargo run --release --bin uniffi-bindgen -- generate \
		--library target/release/libromcal_uniffi.$(LIB_EXT) \
		--language kotlin \
		--out-dir bindings/kotlin/src/main/kotlin/com/romcal/ffi

# Copy native library to resources directory
copy-native-lib: build-rust
	@echo "Copying native library to resources..."
	mkdir -p src/main/resources/$(RESOURCE_DIR)
	cp ../../target/release/libromcal_uniffi.$(LIB_EXT) src/main/resources/$(RESOURCE_DIR)/

# Generate Kotlin types with Typeshare
generate-types:
	@echo "Generating Kotlin types with Typeshare..."
	cd ../.. && typeshare --config-file typeshare.toml ./core/src --lang=kotlin \
		--output-file=bindings/kotlin/src/main/kotlin/com/romcal/types/Types.kt
	@echo "Kotlin types generated (TypesCompat.kt provides compatibility types)"

# Build Kotlin package
build: generate-bindings generate-types copy-native-lib
	@echo "Building Kotlin package..."
	./gradlew build -x test

# Run tests
test: build
	@echo "Running tests..."
	./gradlew test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	./gradlew clean
	rm -rf src/main/kotlin/com/romcal/ffi/
	rm -rf src/main/resources/darwin-*/
	rm -rf src/main/resources/linux-*/
	rm -rf src/main/resources/win32-*/
