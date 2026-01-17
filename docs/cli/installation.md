---
sidebar_position: 2
---

# Installation

## Requirements

- [Rust](https://rustup.rs/) 1.85 or later

## From Source

```bash
# Clone the repository
git clone https://github.com/romcal/romcal.git
cd romcal

# Build and install
cargo install --path cli

# Or run directly without installing
cargo run -p romcal-cli -- date easter_sunday 2025
```

## Binary Location

After building, the binary is located at:

- **Development**: `target/debug/romcal`
- **Release**: `target/release/romcal`

## Shell Completion

Generate completion scripts for your shell:

```bash
# Bash
romcal completions bash > ~/.bash_completion.d/romcal

# Zsh
romcal completions zsh > ~/.zfunc/_romcal

# Fish
romcal completions fish > ~/.config/fish/completions/romcal.fish

# PowerShell
romcal completions powershell >> $PROFILE
```

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`

## Verify Installation

```bash
romcal --version
romcal --help
```
