# Get all the directories that contain a Cargo.toml file and exclude the core directory
SRC_DIRS := $(filter-out core,$(patsubst %/Cargo.toml,%,$(wildcard */Cargo.toml)))

.PHONY: all
all: $(SRC_DIRS)


.PHONY: $(SRC_DIRS)
$(SRC_DIRS):
	@echo "Building $@..." 
	cd $@ && wasm-pack build --target bundler -d ../pkg/$@
	@echo "Done!"




clean:
	rm -rf pkg
	rm -rf target
