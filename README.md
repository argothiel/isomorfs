Install:
```sh
make install
```

Usage:
```sh
mount -t isomorfs ISO_IMAGE MOUNT_POINT
# modify the files
umount MOUNT_POINT
```

E.g.:
```sh
mount -t isomorfs MyFavoriteImage.iso /mnt/image
```

Uninstall:
```sh
make uninstall
```

Test:
```
cargo test
```

Build (dev):
```
cargo build
```
