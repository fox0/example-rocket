SOURCES := $(shell find src/ -name '*.rs')

all: build

build: $(SOURCES)
	cargo build --release

build-win64: $(SOURCES)
	cargo build --target x86_64-pc-windows-gnu --release

clean:
	cargo clean
