#!/usr/bin/sh

cargo build --release &&
  avr-strip roman.elf &&
  doas ravedude nano -P /dev/ttyUSB0 --baudrate 57600 roman.elf
