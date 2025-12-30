set shell := ["bash", "-c"]
set export

RUST_LOG := "debug"

# Show available commands
default:
    @just --list --justfile {{justfile()}}

test: doc-test
	cargo test --all-targets --all-features --workspace
	cargo test --all-targets --workspace --no-default-features
	cargo test --all-targets --workspace --features "all"

doc-test:
	cargo test --no-fail-fast --doc --all-features --workspace

# Update the generated rust code from the protobuf files
update: 
    buf generate https://github.com/openfga/api#format=git

# Update test-sample-stores
update-tests:
    git clone git@github.com:openfga/sample-stores.git tmp/sample-stores
    find tmp/sample-stores/stores -type f \( -name "*.mod" -o -name "*.fga" \) -exec sh -c 'for file; do store_name=$(basename $(dirname $file)); mkdir -p tests/sample-store/$store_name; cp $file tests/sample-store/$store_name/; done' sh {} +
    rm -rf tmp/sample-stores
    mv tests/sample-store/issue-tracker tests/sample-store/modular/issue-tracker

tests-to-json:
    find tests/sample-store -type f -name "*.mod" -exec sh -c 'for file; do store_name=$(basename $(dirname $file)); mkdir -p tests/sample-store/$store_name; fga model transform --file $file > tests/sample-store/$store_name/schema.json; done' sh {} +
    find tests/sample-store -type f -name "model.fga" -exec sh -c 'for file; do store_name=$(basename $(dirname $file)); mkdir -p tests/sample-store/$store_name; fga model transform --file $file > tests/sample-store/$store_name/schema.json; done' sh {} +

update-model-manager-json:
    fga model transform --file tests/model-manager/v1.0/model.fga > tests/model-manager/v1.0/schema.json
    fga model transform --file tests/model-manager/v1.1/model.fga > tests/model-manager/v1.1/schema.json

# Run cargo doc
doc $RUSTDOCFLAGS="-D warnings":
    cargo doc --all --no-deps

# Run cargo doc on all crates and open the docs in your browser
doc-open $RUSTDOCFLAGS="-A missing_docs":
    cargo doc --all --no-deps --open

# Substitute BIN for your bin directory.
# Substitute VERSION for the current released version.
install-buf:
    #!/usr/bin/env sh
    BIN="/usr/local/bin" && \
    VERSION="1.30.1" && \
    curl -sSL \
    "https://github.com/bufbuild/buf/releases/download/v${VERSION}/buf-$(uname -s)-$(uname -m)" \
    -o "${BIN}/buf" && \
    chmod +x "${BIN}/buf"

[private]
fmt:
    cargo +nightly-2025-12-25 fmt --all

check-format:
	cargo +nightly-2025-12-25 fmt --all -- --check

check-clippy:
	cargo clippy --all-features --workspace -- -D warnings
	cargo clippy --workspace -- -D warnings
	cargo clippy --workspace --no-default-features -- -D warnings

check-cargo-sort:
	cargo sort -c -w

fix-format:
    cargo clippy --all-targets --all-features --workspace --fix --allow-staged
    cargo +nightly-2025-12-25 fmt --all
    cargo sort -w

check: check-format check-clippy check-cargo-sort
