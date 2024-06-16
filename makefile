default: main

build:
	cd RaTuS && cargo build && cd ..

clean:
	cd RaTuS && cargo clean && cd ..

test: 
	cd RaTuS && cargo test && cd ..

install:
	curl https://sh.rustup.rs -sSf | sh
