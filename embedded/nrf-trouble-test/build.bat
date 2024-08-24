cargo +nightly build --release
cd target\thumbv7em-none-eabihf\release
..\..\..\utils\arm-none-eabi-objcopy.exe -Oihex %1 %1.hex
python3 ..\..\..\utils\uf2conv.py %1.hex -c -b 0x26000 -f 0xADA52840
copy flash.uf2 E:\
