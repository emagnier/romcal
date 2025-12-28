//! TypeScript type generator for Romcal types using ts-rs.
//!
//! This binary generates TypeScript type definitions from Rust types,
//! enabling type-safe interop between Rust and TypeScript.
//!
//! ts-rs exports types via #[ts(export)] attribute during compilation.
//! Run: cargo test --features ts-bindings to generate the types.

fn main() {
    println!("TypeScript types are exported via ts-rs during test compilation.");
    println!("Run: cargo test --features ts-bindings");
    println!("Types will be written to the directory specified in ts-rs configuration.");
}
