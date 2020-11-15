.PHONY: all build clean

all: build

build:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
	wasm-opt -Oz ./target/wasm32-unknown-unknown/release/*.wasm -o ./contract.wasm
	cat ./contract.wasm | gzip -9 > ./contract.wasm.gz 

alternatively-build-with-docker:
	docker run --rm -it -v $(shell pwd):/contract enigmampc/secret-contract-optimizer

clean:
	cargo clean
	-rm -f ./contract.wasm ./contract.wasm.gz

