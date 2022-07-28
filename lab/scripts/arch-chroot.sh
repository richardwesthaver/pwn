#!/usr/bin/sh
src=http://mirrors.mit.edu/archlinux/iso/2021.11.01/archlinux-bootstrap-2021.11.01-x86_64.tar.gz
archive=/tmp/archlinux-bootstrap-2021.11.01-x86_64.tar.gz
image=/tmp/image.raw
mountpoint=/mnt/arch

if [[ ! -f $archive ]]; then
    wget $src -O $archive
fi

mkdir $mountpoint

qemu-img create -f raw $image 16G
loop=$(sudo losetup --show -f -P $image)
sudo parted $loop mklabel msdos
sudo parted -a optimal $loop mkpart primary 0% 100%
sudo parted $loop set 1 boot on
loopp=${loop}p1
sudo mkfs.ext4 $loopp
sudo mount $loopp $mountpoint
sudo tar xf $archive -C $mountpoint --strip-components 1

sudo $mountpoint/bin/arch-chroot $mountpoint /bin/bash <<EOL
set -v

echo 'Server = http://mirror.erickochen.nl/archlinux/\$repo/os/\$arch' >> /etc/pacman.d/mirrorlist

pacman-key --init
pacman-key --populate archlinux

pacman -Syu --noconfirm
pacman -S --noconfirm base linux linux-firmware mkinitcpio dhcpcd syslinux
systemctl enable dhcpcd

# Standard Arch Linux Setup
ln -sf /usr/share/zoneinfo/EST /etc/localtime
hwclock --systohc
echo en_US.UTF-8 UTF-8 >> /etc/locale.gen
locale-gen
echo LANG=en_US.UTF-8 > /etc/locale.conf
echo arch-qemu > /etc/hostname
echo -e '127.0.0.1  localhost\n::1  localhost' >> /etc/hosts

# Create an initramfs without autodetect, because this breaks with the
# combination host/chroot/qemu
linux_version=\$(ls /lib/modules/ | sort -V | tail -n 1)
mkinitcpio -c /etc/mkinitcpio.conf -S autodetect --kernel \$linux_version -g /boot/initramfs-linux-custom.img

# Setup syslinux
mkdir /boot/syslinux
cp /usr/lib/syslinux/bios/*.c32 /boot/syslinux/
extlinux --install --device $loopp /boot/syslinux
dd bs=440 count=1 conv=notrunc if=/usr/lib/syslinux/bios/mbr.bin of=$loop
# Customize syslinux config
uuid=\$(blkid -o value -s UUID $loopp)
sed -i '1i SERIAL 0 115200' /boot/syslinux/syslinux.cfg
sed -i "s/APPEND root=\/dev\/sda3/APPEND console=tty0 console=ttyS0,115200 root=UUID=\$uuid/g" /boot/syslinux/syslinux.cfg
sed -i "s/INITRD ..\/initramfs-linux.img/INITRD ..\/initramfs-linux-custom.img/" /boot/syslinux/syslinux.cfg
EOL

sudo umount $mountpoint
sudo losetup -d $loop
