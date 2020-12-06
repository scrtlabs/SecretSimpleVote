.PHONY: all build clean

all: build

build:
	RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown --locked
	wasm-opt -Oz ./target/wasm32-unknown-unknown/release/secret_voting.wasm -o ./contract.wasm
	cat ./contract.wasm | gzip -9 > ./contract.wasm.gz 

alternatively-build-with-docker:
	docker run --rm -it -v $(shell pwd):/contract enigmampc/secret-contract-optimizer

# alias secretcli='docker exec -it secretdev secretcli'
start-local-chain: # CTRL+C to stop
	docker run -it --rm -p 26657:26657 -p 26656:26656 -p 1337:1337 -v $(shell pwd):/root/code --name secretdev enigmampc/secret-network-sw-dev:v1.0.4

store-contract:
	docker exec -it secretdev secretcli tx compute store /root/code/contract.wasm.gz --from a --gas 10000000 -b block -y

clean:
	cargo clean
	-rm -f ./contract.wasm ./contract.wasm.gz

