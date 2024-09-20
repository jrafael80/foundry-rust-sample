#!/usr/bin/make -f

# Default target is build
default: build

# Define variables
CARGO=cargo
CRATES_FOLDER=crates
CONTRACTS_PATH=./contracts
BINDINGS_FOLDER=bindings
BINDINGS_CRATES_FOLDER=$(CRATES_FOLDER)/$(BINDINGS_FOLDER)
BINDINGS_OUT_PATH=$(CONTRACTS_PATH)/out/$(BINDINGS_FOLDER)
MAIN_CONTRACT=src/MultiSend.sol:MultiSend

# Target for generating bindings
bindings:
	rm -rf $(BINDINGS_CRATES_FOLDER)
	rm -rf $(BINDINGS_OUT_PATH)
	
# Generate new bindings
	@forge bind --root $(CONTRACTS_PATH) --crate-name $(BINDINGS_FOLDER)
	
# Move bindings to the correct location
	@mv -f $(BINDINGS_OUT_PATH) $(CRATES_FOLDER)

# Target for building the project
build: bindings
	@$(CARGO) build
	
# Target for building the project in release mode
build-release: bindings
	@$(CARGO) build --release

# Target for cleaning the project
clean:
	@forge clean --root $(CONTRACTS_PATH)
	@$(CARGO) clean

# Target for formatting the code
fmt:
	@forge fmt --check --root $(CONTRACTS_PATH)
	@$(CARGO) fmt

# Target for running tests
test:
	@forge test --root $(CONTRACTS_PATH)
	@$(CARGO) test

# Target for installing forge dependencies
setup:
	@forge install --root $(CONTRACTS_PATH)


# Declare phony targets
.PHONY: build build-release clean fmt bindings


run-local-node:
	anvil --block-time 10

verify-contract:
	forge verify-contract 0xdfbfbdfb29de74e0a3a588abe4be512e03a54744 —verifier=blockscout --verifier-url=https://eth-sepolia.blockscout.com/api/

deploy:
	forge create --root $(CONTRACTS_PATH) --rpc-url http://127.0.0.1:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 $(MAIN_CONTRACT)

ethernal:
	npx --yes ethernal listen

advisories:
	cargo deny -c always --locked -L info check advisories