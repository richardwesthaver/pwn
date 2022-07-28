#!/bin/sh
#
# this script requires an installation ISO (win10_x64.iso), the
# virtio-windows-guest drivers (win10.iso), and a fresh disk image to
# install to (win10.img).
#
# the graphic installer will boot and you then need to load the virtio
# drivers, at which point you can install as usual.
exec qemu-system-x86_64 -enable-kvm \
        -cpu host \
        -cdrom win10_x64.iso \
        -drive file=win10.img,if=virtio \
        -drive file=win10.iso,index=1,media=cdrom \
        -net nic,model=virtio -net user \
        -vga qxl \
        -m 4G \
        -monitor stdio \
        -name "Windows" \
        "$@"
