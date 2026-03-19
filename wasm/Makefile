all: build

build:
	@echo "Building wasm..." 
	wasm-pack build --target bundler
	@echo "Done!"

clean:
	rm -rf pkg
	rm -rf target
