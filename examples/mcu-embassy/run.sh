#!/bin/bash
set -e

# cargo objcopy --release -- -O ihex out.hex

rm -f $1.elf
mv $1 $1.elf

# STM32_Programmer_CLI only works if the file has an elf extension
$HOME/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI --connect port=JLINK mode=UR reset=HWrst freq=4000 --download $1.elf --go
 
probe-rs attach --chip STM32U5G9ZJTxQ $1.elf
# probe-rs attach --chip STM32U5G9ZJTxQ ./target/thumbv8m.main-none-eabihf/release/eth.elf