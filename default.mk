BUILD_TYPE ?= release

BINDING_NAME = jieba_rustlib

CARGO_OUTPUTS += lib$(BINDING_NAME).so

RUST_OUTPUTS = $(CARGO_OUTPUTS) $(BINDING_NAME).h

