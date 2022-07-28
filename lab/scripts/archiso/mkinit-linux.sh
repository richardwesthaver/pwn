#!/usr/bin/bash
# download, verify, unpack, and supply config to linux-5.x from scratch.
# this takes a while.
linux_version="5.15"
build_dir="$STAMP/linux-$linux_version"

mkdir -pv $build_dir
cd $build_dir
# download
wget https://www.kernel.org/pub/linux/kernel/v5.x/linux-$linux_version.tar.xz
# verify
wget https://www.kernel.org/pub/linux/kernel/v5.x/linux-$linux_version.tar.sign
gpg --list-packets linux-$linux_version.tar.sign
gpg --recv-keys
unxz linux-$linux_version.tar.xz
gpg --verify linux-$linux_version.tar.sign linux-$linux_version.tar
# unpack
tar -xvf linux-$linux_version.tar
# supply
cd linux-$linux_version/
make mrproper -j
zcat /proc/config.gz > .config
make localmodconfig
