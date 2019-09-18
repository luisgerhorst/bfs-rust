# KO_DIR ?= /lib/modules/5.0.19-yocto-standard/extra
KO_DIR ?= .

LOOP_IMAGE ?= loop0-img
LOOP_DEV ?= /dev/loop0
LOOP_MOUNTPOINT ?= loop0-mp

# Module

insmod:
	insmod $(KO_DIR)/bfsrust.ko

rmmod:
	rmmod --syslog bfsrust # --syslog quiets the "could not open builtin file" error

# Image

$(LOOP_IMAGE):
	touch $(LOOP_IMAGE) \
	&& dd bs=4096 count=2000 if=/dev/zero of="$(LOOP_IMAGE)" \
	&& losetup $(LOOP_DEV) $(LOOP_IMAGE)

$(LOOP_MOUNTPOINT):
	mkdir $(LOOP_MOUNTPOINT)

erase:
	losetup -d $(LOOP_DEV)
	rm $(LOOP_IMAGE) \
	; rm -rfd $(LOOP_MOUNTPOINT)

hexdump:
	hexdump $(LOOP_DEV) -C

# Mounting

mount: $(LOOP_IMAGE) $(LOOP_MOUNTPOINT)
	mount -o loop -t bfs-rust $(LOOP_DEV) $(LOOP_MOUNTPOINT)

umount:
	umount $(LOOP_MOUNTPOINT)

remount: | umount mount

# Load/Unload/Reload

load: | insmod mount
unload: | umount rmmod
reload: | unload load

erase-reload: | umount erase rmmod load
