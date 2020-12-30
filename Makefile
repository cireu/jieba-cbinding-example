include default.mk

CC = gcc

CFLAGS += -I$(BINDING_NAME) -L$(BINDING_NAME)

ifeq ($(BUILD_TYPE), debug)
	CFLAGS += -O0 -g3
endif

all: main

$(RUST_OUTPUTS):
	$(MAKE) -C $(BINDING_NAME) $@

main: main.c lib$(BINDING_NAME).so
	$(CC) $< $(CFLAGS) -Wl,-rpath=$(BINDING_NAME) -l$(BINDING_NAME) -o $@

.PHONY: all
