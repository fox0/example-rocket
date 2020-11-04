MAIN = rocket-rust
SOURCES := $(shell find src/ -name '*.rs')
PREFIX := $(MAIN)-$(shell date +'%Y%m%d')

all: $(PREFIX)-linux64.zip $(PREFIX)-win64.zip

$(PREFIX)-linux64.zip: target/release/$(MAIN)
	zip -j $@ $<
	zip -u $@ Rocket.toml
	zip -u -r $@ static
	zip -u -r $@ templates

$(PREFIX)-win64.zip: target/x86_64-pc-windows-gnu/release/$(MAIN).exe
	zip -j $@ $<
	zip -u $@ Rocket.toml
	zip -u -r $@ static
	zip -u -r $@ templates

target/release/$(MAIN): $(SOURCES)
	cargo build --release

target/x86_64-pc-windows-gnu/release/$(MAIN).exe: $(SOURCES)
	cargo build --target x86_64-pc-windows-gnu --release

clean:
	cargo clean
	rm -f *.zip
