mount.isomorfs: main.rs
	rustc -o $@ $^

install:
	install -m 755 mount.isomorfs /sbin/mount.isomorfs

uninstall:
	rm /sbin/mount.isomorfs
