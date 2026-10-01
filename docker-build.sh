#! /bin/sh
# In-container ARM build for Kobo (runs inside plato-build docker container).
# Strategy: noble's gcc-13 cross package + Debian jessie glibc-2.19 sysroot (/jessie,
# mounted ro) so emitted symbol versions stay <= GLIBC_2.19, matching the official
# plato binary's profile (<= GLIBC_2.18, same 11 NEEDED libs).
set -e
J=/jessie

# Compiler/linker wrapper: jessie sysroot for headers+libc, jessie lib dir searched
# FIRST so its libc.so/libm.so version maps cap symbol versions; gcc-13's own
cat > /usr/local/bin/arm-link <<EOF
#! /bin/sh
J=$J
# Trailing -fuse-ld=bfd overrides rustc's default -fuse-ld=lld, which rejects
# the official libmupdf.so's legacy linker-script symbols (__bss_start__ etc.).
exec arm-linux-gnueabihf-gcc \\
	--sysroot="\$J" \\
	-isystem "\$J/usr/include/arm-linux-gnueabihf" \\
	-isystem "\$J/usr/include" \\
	-B"\$J/usr/lib/arm-linux-gnueabihf" \\
	-B/usr/lib/gcc-cross/arm-linux-gnueabihf/13 \\
	-L"\$J/usr/lib/arm-linux-gnueabihf" \\
	-L"\$J/lib/arm-linux-gnueabihf" \\
	"\$@" -fuse-ld=bfd
EOF
chmod +x /usr/local/bin/arm-link

export PATH=/root/.cargo/bin:$PATH
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-link
export CC_arm_unknown_linux_gnueabihf=arm-link
export CXX_arm_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++

cd /plato/mupdf_wrapper
TARGET_OS=Kobo CC=arm-link AR=arm-linux-gnueabihf-ar ./build.sh
cd /plato

cargo build --release --target=arm-unknown-linux-gnueabihf -p plato

P=target/arm-unknown-linux-gnueabihf/release/plato
file "$P"
echo '--- symbol versions ---'
arm-linux-gnueabihf-readelf -V "$P" | grep -oE '(GLIBC|GCC|CXXABI|GLIBCXX)_[0-9.]+' | sort -uV | tr '\n' ' '
echo
echo '--- UND above GLIBC_2.18 (must be empty) ---'
arm-linux-gnueabihf-readelf --dyn-syms -W "$P" | awk '$7=="UND"' | grep -E 'GLIBC_(2\.(19|2[0-9]|3[0-9]|4[0-9]))' || true
