#/bin/bash

PATH_TO_EFI="$1"
cd `dirname $0`
cd ..
mkdir -p mnt/EFI/BOOT
cp ${PATH_TO_EFI} mnt/EFI/BOOT/BOOTX64.EFI
qemu-system-x86_64 \
    -bios third-party/ovmf/RELEASEX64_OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    -serial stdio
