cargo +nightly build --release
cd target\thumbv7em-none-eabihf\release
..\..\..\utils\arm-none-eabi-objcopy.exe -Oihex nrf52840-ws2812-test nrf52840-ws2812-test.hex
python3 ..\..\..\utils\uf2conv.py nrf52840-ws2812-test.hex -c -b 0x27000 -f 0xADA52840
copy flash.uf2 E:\
