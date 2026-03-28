# Get all the directories that contain a Cargo.toml file and exclude the core directory
SRC_DIRS := $(filter-out core,$(patsubst %/Cargo.toml,%,$(wildcard */Cargo.toml)))

.PHONY: all
all: $(SRC_DIRS)

release: $(addprefix release-,$(SRC_DIRS))


.PHONY: $(SRC_DIRS)
$(SRC_DIRS):
	@echo "Building $@..." 
	wasm-pack build $@ --target bundler -d ../pkg/$@
	@echo "Done!"

.PHONY: $(addprefix release-,$(SRC_DIRS))
$(addprefix release-,$(SRC_DIRS)):
	@echo "Building $(patsubst release-%,%,$@) in release mode..."
	wasm-pack build $(patsubst release-%,%,$@) --target bundler -d ../pkg/$(patsubst release-%,%,$@)  --release --mode force
	@echo "Done!"


clean:
	rm -rf pkg
	rm -rf target
