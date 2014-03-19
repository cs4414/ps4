arch=arm
RUST_ROOT := /usr
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/

export RUST_ROOT
export LLVM_ROOT
export GCC_PREFIX

all:
	@$(MAKE) all -C arch/$(arch)/

%:
	@$(MAKE) $* -C arch/$(arch)/
