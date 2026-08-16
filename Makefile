ifeq ($(MAKECMDGOALS),)
  $(error Use 'make install' or 'make uninstall')
endif

.PHONY: install uninstall

install:
	cargo build --release
	install -C -m 755 target/release/isomorfs /sbin/mount.isomorfs

uninstall:
	rm /sbin/mount.isomorfs
