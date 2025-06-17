.PHONY: dev release

all: dev

dev:
	@echo "Building development version..."
	@cargo build

release:
	@echo "Building release version..."
	@cargo build --release

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
