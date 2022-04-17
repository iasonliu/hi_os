build:
	cargo xbuild
bootimage:
	cargo bootimage
make_usbdev:
	#dd if=target/x86_64-hi_os/debug/bootimage-hi_os.bin of=/dev/sdX && sync
run:
	cargo xrun