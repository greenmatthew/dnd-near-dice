# Default target
.PHONY: all
all: help

.PHONY: build
build:
	@echo "Building dnd-near-dice..."
	cargo build

.PHONY: run
run:
	@echo "Running dnd-near-dice..."
	cargo run

.PHONY: test
test:
	@echo "Running tests..."
	cargo test

.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

.PHONY: clippy
clippy:
	@echo "Running clippy with pedantic warnings..."
	cargo clippy -- -W clippy::pedantic -W clippy::all -W clippy::nursery -D warnings

.PHONY: clippy-fix
clippy-fix:
	@echo "Running clippy with automatic fixes..."
	cargo clippy --fix -- -W clippy::pedantic -W clippy::all -W clippy::nursery

.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make          - Display this help message"
	@echo "  make build    - Compile the project"
	@echo "  make run      - Run the application" 
	@echo "  make test     - Run all tests"
	@echo "  make clippy   - Run Clippy with pedantic warnings"
	@echo "  make clippy-fix - Run Clippy with automatic fixes"
	@echo "  make clean    - Remove build artifacts"
	@echo "  make help     - Display this help message"