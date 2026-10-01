#! /bin/sh
# Deploy the patched plato binary to a Kobo in USBMS (share) mode.
# Rollback: copy plato.bak back to plato.
set -e

SRC=/tmp/plato/target/arm-unknown-linux-gnueabihf/release/plato
[ -x "$SRC" ] || { echo "binary missing: $SRC"; exit 1; }

# Locate a mounted Kobo (partition root containing .adds/plato).
M=$(findmnt -rn -o TARGET | while read -r t; do [ -d "$t/.adds/plato" ] && echo "$t" && break; done)

if [ -z "$M" ]; then
	echo "No mounted partition with .adds/plato found."
	echo "1) On the reader: start Plato, plug in USB, allow 'Share storage via USB?'"
	echo "2) If the host still doesn't auto-mount, find the device with 'lsblk -f'"
	echo "   and mount it, e.g.: udisksctl mount -b /dev/sdXN"
	exit 1
fi

test -d "$M/.adds/plato" || { echo "KFMon layout missing under $M"; exit 1; }
[ -f "$M/.adds/plato/plato.bak" ] || cp -a "$M/.adds/plato/plato" "$M/.adds/plato/plato.bak"
echo "backup: $M/.adds/plato/plato.bak"
install -m 755 "$SRC" "$M/.adds/plato/plato"
sync
echo "installed: $M/.adds/plato/plato"
echo "unmount with: udisksctl unmount -b $(findmnt -rn -no SOURCE --target "$M")"
echo "then eject on the reader; the KFMon icon starts the patched build."
