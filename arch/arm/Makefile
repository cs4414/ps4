SHELL=/bin/bash
TARGET := arm-linux-noeabi

GCC_PREFIX := $(GCC_PREFIX)arm-none-eabi-

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -Z debug-info -Z extra-debug-info

AS := $(GCC_PREFIX)as
LD := $(GCC_PREFIX)ld

LLC := $(LLVM_ROOT)/bin/llc
LLCFLAGS := -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-arm
QEMUFLAGS := -M versatilepb -m 32M -serial stdio

BDIR := ./boot
CORE_LIB := ../../rust-core/core/lib.rs
LCORE := $(BDIR)/$(shell $(RUSTC) --crate-file-name $(CORE_LIB))
OBJS := $(BDIR)/loader.o $(BDIR)/aeabi_runtime.o $(BDIR)/main.o $(BDIR)/core.o
LINK := $(BDIR)/linker.ld $(OBJS)

MODS := $(wildcard */*.rs) $(wildcard ../../kernel/*.rs) $(wildcard ../../kernel/*/*.rs) $(wildcard ../../kernel/*/*/*.rs)

-include ./config.mk
-include $(BDIR)/core.d
-include $(BDIR)/loader.d

.PHONY: all run debug clean

all: $(BDIR)/kernel.bin
	@wc -c $^

# Library rust-core
$(LCORE):
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/core.d $(CORE_LIB) --out-dir $(BDIR)

$(BDIR)/core.bc: $(LCORE)
	cd $(@D); ar -x $(<F) $(@F)

# Compile rustboot
$(BDIR)/main.bc: ../../lib.rs $(MODS) $(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/main.d -L $(BDIR) --emit-llvm ../../lib.rs --out-dir $(BDIR)

%.s: %.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

%.o: %.s
	$(AS) -MD $*.d -g $< -o $@

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) -o $@ -T $^

# kernel (binary image)
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $^ $@

# running
run: $(BDIR)/kernel.bin
	$(QEMU) $(QEMUFLAGS) -kernel $^

debug: $(BDIR)/kernel.elf
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) $(QEMUFLAGS) -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB)"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	# TODO: debug in current window, can't kill panes
	tmux new-w "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-w "$(GDB); tmux kill-w"
endif

clean:
	rm -f $(BDIR)/*.{d,o,bc,rlib,so,ll,embed,elf,bin}
