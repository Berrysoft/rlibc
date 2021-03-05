-include ./config.mk

ARCH		   ?= x86_64
TARGET		   ?= x86_64-unknown-linux-gnu

BDIR			= target
CONFIG		   ?= debug
TARGETDIR       = $(BDIR)/$(TARGET)/$(CONFIG)

LD				= ld.lld
CLANG			= clang
CARGO			= cargo

# add -L $(TARGETDIR)/deps for non-native platforms
CLANGFLAGS		= -target $(TARGET) -I include/rlibc -nostdlib -fno-stack-protector -fno-builtin
CARGOFLAGS		=

ifeq ($(CONFIG), release)
CARGOFLAGS	   += --release
endif

.PHONY: all directories run clean $(TARGETDIR)/libc.a

# Depend on librlibc.a so that cargo will create targets
all: directories $(TARGETDIR)/test

directories:
	mkdir -p $(TARGETDIR)

$(TARGETDIR)/libc.a:
	$(CARGO) build $(CARGOFLAGS) --target=$(TARGET)

$(TARGETDIR)/crt0.o: crt/$(TARGET)/crt0.s
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test.o: test.c include/rlibc/libc.h
	$(CLANG) $(CLANGFLAGS) -c $< -o $@

$(TARGETDIR)/test: $(TARGETDIR)/crt0.o $(TARGETDIR)/test.o $(TARGETDIR)/libc.a
	$(LD) -e start $^ -o $@

run: all
	$(TARGETDIR)/test

clean: directories
	$(CARGO) clean
