default: main

build:
	cd RaTuS && cargo build && cd ..

clean:
	cd RaTuS && cargo clean && cd ..

test: 
	cd RaTuS && cargo test && cd ..

install:
	curl https://sh.rustup.rs -sSf | sh

start_node_1:
	cd RaTuS && cargo run --bin start_node -- --id 1 --http-addr 127.0.0.1:21001

start_node_2:
	cd RaTuS && cargo run --bin start_node -- --id 2 --http-addr 127.0.0.1:21002

start_node_3:
	cd RaTuS && cargo run --bin start_node -- --id 3 --http-addr 127.0.0.1:21003

test_app:
	cd RaTuS && cargo run --bin test_app