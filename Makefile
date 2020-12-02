.PHONY: all build clean

all: build

build:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
	wasm-opt -Oz ./target/wasm32-unknown-unknown/release/secret_voting.wasm -o ./contract.wasm
	cat ./contract.wasm | gzip -9 > ./contract.wasm.gz 

alternatively-build-with-docker:
	docker run --rm -it -v $(shell pwd):/contract enigmampc/secret-contract-optimizer

start-local-chain: # CTRL+C to stop
	docker run -it --rm -p 26657:26657 -p 26656:26656 -p 1317:1317 -v $(shell pwd):/root/code --name secretdev enigmampc/secret-network-sw-dev:v1.0.2

clean:
	cargo clean
	-rm -f ./contract.wasm ./contract.wasm.gz

