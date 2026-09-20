# List available recipes
default:
    @just --list

# Build the library plus all examples/tests
build:
    cargo build --all-targets

# Run all tests (unit, integration, doc)
test:
    cargo test

# Lint with clippy, denying all warnings (matches CI)
clippy:
    cargo clippy --all-targets -- -D warnings

# Format the code
fmt:
    cargo fmt

# Check formatting without changing files
fmt-check:
    cargo fmt -- --check

# Run everything CI runs, locally
ci: build test clippy fmt-check

# Run an example, e.g. `just example demo`
example name:
    cargo run --example {{name}}

# Dry-run what `cargo publish` would upload, without actually publishing
publish-dry-run:
    cargo publish --dry-run

# Upload the crates.io token as a GitHub Actions repo secret
set-cargo-token:
    gh secret set CARGO_REGISTRY_TOKEN
