.PHONY: format
format:
	@echo "[bash_tokenizer] Formatting code..."
	cargo fmt

.PHONY: build
build:
	@echo "[bash_tokenizer] Building library..."
	cargo build

.PHONY: test
test:
	@echo "[bash_tokenizer] Running unit tests..."
	cargo test