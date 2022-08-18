#!/bin/sh
#
# this script requires:
# - a Windows disk image: https://www.microsoft.com/software-download/windows11
# - virtio-windows-guest drivers: https://github.com/virtio-win/virtio-win-pkg-scripts
# - a fresh disk image to install to
#
# the graphic installer will boot and you then need to load the virtio
# drivers, at which point you can install as usual.
exec qemu-system-x86_64 -enable-kvm \
        -cpu host \
        -cdrom win10_x64.iso \
        -drive file=win11.img,if=virtio \
        -drive file=win11.iso,index=1,media=cdrom \
        -net nic,model=virtio -net user \
        -vga qxl \
        -m 4G \
        -monitor stdio \
        -name "Windows" \
        "$@"
