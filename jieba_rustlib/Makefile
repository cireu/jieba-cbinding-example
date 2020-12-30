include ../default.mk

CARGO = cargo
BUILD_DIST_DIR = target/$(BUILD_TYPE)

$(CARGO_OUTPUTS):
ifeq ($(BUILD_TYPE), debug)
	$(CARGO) build
else
	$(CARGO) build --$(BUILD_TYPE)
endif
	cp $(BUILD_DIST_DIR)/$@ ./

$(BINDING_NAME).h: src/lib.rs
	cbindgen -o $@

# Always rebuild, Rust will do incremental compiling automatically
.PHONY: $(CARGO_OUTPUTS)
