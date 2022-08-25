#!/bin/sh

# this script requires:
# - $1: a Windows disk image: https://www.microsoft.com/software-download/windows11
# - $2: a fresh disk image to install to
# - $3: virtio-windows-guest drivers: https://github.com/virtio-win/virtio-win-pkg-scripts
#
# the graphic installer will boot and you then need to load the virtio
# drivers, at which point you can install as usual.

exec qemu-system-x86_64 \
     -accel hvf \
#    -enable-kvm \
     -cpu host \
     -hda $1 \
     -boot d \
     -cdrom $2 \
#    -drive file=$3,if=virtio \
#    -net nic,model=virtio -net user \
#    -vga qxl \
     -m 4G \
     -monitor stdio \
     -name "Windows" \
     "$@"
