_default:
	@just --list

# Format
fmt:
    @cargo fmt --all

# Lint
lint:
	@cargo fmt --all -- --check
	@cargo clippy --all --tests -- -D warnings

# Test
test:
	@cargo test --all

# Test with coverage
test-cov:
	@cargo tarpaulin

# Generate documentation
doc:
	@cargo doc

# Run one day
run-day day:
	@cargo run -q -- run-day {{ day }}

# Run all days
run-all:
	@cargo run -q -- run-all