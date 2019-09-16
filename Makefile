obj-m := bfsrust.o
bfsrust-objs := bfs_rust.rust.o
KERNEL_SRC ?= /lib/modules/$(shell uname -r)/build
SRC := $(shell pwd)

# Yocto seems to prevent usage of *.o files in SRC_URI in some way. Therefore
# secretly get the .o file into Yocto/Poky as .rust_o and then rename it.
%.rust_o: target/x86_64-linux-kernel/debug/lib%.a
	$(LD) -r -o $@ --whole-archive $<

%.rust.o: %.rust_o
	mv $< $@

all: bfs_rust.rust.o
	$(MAKE) -C $(KERNEL_SRC) M=$(SRC)

modules_install:
	$(MAKE) -C $(KERNEL_SRC) M=$(SRC) modules_install

clean:
	$(MAKE) -C $(KERNEL_SRC) M=$(SRC) clean
