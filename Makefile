.PHONY: build run test clean install

BINARY_NAME=rust-agent-tool

build:
	cargo build --release

run:
	cargo run --release

test:
	cargo test

clean:
	cargo clean

install: build
	mkdir -p /usr/local/bin
	cp target/release/$(BINARY_NAME) /usr/local/bin/$(BINARY_NAME)
	@echo "✅ Binary installed to /usr/local/bin/$(BINARY_NAME)"

# Helper to download the recommended model
download-model:
	mkdir -p models
	curl -L https://huggingface.co/lmstudio-community/gemma-4-E4B-it-GGUF/resolve/main/gemma-4-e4b-it-Q4_K_M.gguf -o models/gemma-4-e4b.gguf
	@echo "✅ Gemma 4 e4b downloaded to ./models"

setup:
	mkdir -p knowledge models
	@if [ ! -f appliance.toml ]; then \
		echo 'knowledge_path = "./knowledge"\nmodels_path = "./models"\nrates_path = "rates.toml"\n\n[server]\nport = 6789\nhost = "127.0.0.1"' > appliance.toml; \
	fi
	@echo "✅ Environment setup complete."
