NAME=vsim
WASM=target/wasm32-unknown-unknown/release/
DEST=public/

.PHONY: build-wasm
build-wasm:
	@cargo build --target wasm32-unknown-unknown --release
	@mv $(WASM)$(NAME).wasm $(DEST)

run-wasm:
	@$(MAKE) build-wasm
	@basic-http-server $(DEST)