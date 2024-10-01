FIGURES =$(wildcard assets/diagrams/flowcharts/*.d2)
SVG_OUTPUT_DIR = assets/diagrams/svg
FEATURES = --features v2_39

# Build the library
all:
	cargo build $(FEATURES)

# Build figures and diagrams
fig-build: $(FIGURES)
	./scripts/build-diagrams --output $(SVG_OUTPUT_DIR) $?

# Build the library documentation
doc:
	cargo doc $(FEATURES) --no-deps -p rsblkid-sys -p rsblkid

# Rebuild documentation and diagrams
doc-rebuild: fig-build doc

# Run unit/integration tests
test:
	cargo nextest run $(FEATURES)

# Run doc tests
doctest:
	cargo test $(FEATURES) --doc

# Run all tests
fulltest: test doctest
