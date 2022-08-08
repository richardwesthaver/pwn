#!/usr/bin/env sh
# boots into uefi shell
pushd lab/scratch
qemu-system-x86_64 -bios ovmf-x64/img/OVMF-pure-efi.fd -hda fat:box/mnt/hda -net none

popd
