all: run

build:
	cargo b --release

run:
	cargo r --release

debug:
	cargo r

clean:
	cargo clean
