#!/bin/bash

PATH_TO_EFI="$1"
QEMU_BASE_OPTIONS=" \
    -bios third-party/ovmf/RELEASEX64_OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -chardev stdio,id=char_com1,mux=on,logfile=log/com1.txt \
    -serial chardev:char_com1 \
"
QEMU_RUN_OPTIONS=" \
"
QEMU_TEST_OPTIONS=" \
    -display none \
"

if [[ "${PATH_TO_EFI}" =~ lemonos.efi$ ]]; then
    # when `cargo run`
    QEMU_OPTIONS="$QEMU_BASE_OPTIONS $QEMU_RUN_OPTIONS"
else
    # when `cargo test`
    QEMU_OPTIONS="$QEMU_BASE_OPTIONS $QEMU_TEST_OPTIONS"
fi

cd `dirname $0`
cd ..
mkdir -p mnt/EFI/BOOT
cp ${PATH_TO_EFI} mnt/EFI/BOOT/BOOTX64.EFI
qemu-system-x86_64 $QEMU_OPTIONS

# Convert Exit Code
exit_code=$?
case "${exit_code}" in
    "0")
        # QEMU exited successfully
        exit 0;;
    "1")
        # QEMU exited abnormally
        exit 1;;
    "33")
        # `cargo test` success
        exit 0;;
    "35")
        # `cargo test` failed
        exit 1;;
    *)
        exit $exit_code;;
esac
