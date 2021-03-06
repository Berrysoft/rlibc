-include ./config.mk

ARCH		   ?= x86_64
TARGET		   ?= x86_64-unknown-linux-gnu

BDIR			= target
CONFIG		   ?= debug
TARGETDIR       = $(BDIR)/$(TARGET)/$(CONFIG)

CLANG			= clang
CARGO			= cargo

# add -L $(TARGETDIR)/deps for non-native platforms
CLANGFLAGS		= -target $(TARGET) -I include/rlibc -nostdlib -fno-stack-protector -fno-builtin -fvisibility=hidden
CARGOFLAGS		=

ifeq ($(CONFIG), release)
CLANGFLAGS	   += -flto=thin -O3
CARGOFLAGS	   += --release
export RUSTFLAGS = -C linker-plugin-lto
endif

.PHONY: all directories run clean $(TARGETDIR)/libc.a

# Depend on librlibc.a so that cargo will create targets
all: directories $(TARGETDIR)/test

directories:
	mkdir -p $(TARGETDIR)

$(TARGETDIR)/libc.a:
	$(CARGO) build $(CARGOFLAGS) --target=$(TARGET)

$(TARGETDIR)/test.o: test.c include/rlibc/libc.h
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/test.o $(TARGETDIR)/libc.a
	$(CLANG) $(CLANGFLAGS) -fuse-ld=lld $^ -o $@

run: all
	$(TARGETDIR)/test

clean: directories
	$(CARGO) clean
