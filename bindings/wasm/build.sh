#!/bin/bash

# Build script for romcal WASM bindings

echo "🔨 Building Romcal WASM bindings..."

# Going to the core/romcal directory
cd ../../core/romcal

# Compiling WASM directly to bindings/wasm/pkg
echo "📦 Compiling WASM..."
wasm-pack build --target nodejs --out-dir ../../bindings/wasm/pkg

echo "✅ Build completed successfully!"
echo ""
echo "To test the build:"
echo "  cd ../../bindings/wasm"
echo "  npm test"
echo ""
echo "To run the example:"
echo "  npx tsx example.ts"
