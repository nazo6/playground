#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <ptx|prx>"
  exit 1
elif [ $1 == "ptx" ]; then
  BIN="ptx"
  VID="CAFE"
  PID="BBBB"
elif [ $1 == "prx" ]; then
  BIN="prx"
  VID="CAFE"
  PID="CCCC"
else
    echo "Usage: $0 <ptx|prx>"
    exit 1
fi

cargo run --release --bin "$BIN"

COM_PORT=$(pwsh.exe -NoProfile -Command \
"Get-CimInstance Win32_PnPEntity |
Where-Object { \$_.PNPDeviceID -match 'VID_${VID}&PID_${PID}' -and \$_.Name -match 'COM\d+' } |
ForEach-Object { [regex]::Match(\$_.Name, 'COM(\d+)').Groups[1].Value } |
Select-Object -First 1
"
)
COM_PORT=$(echo "$COM_PORT" | tr -d '\r')

echo "COM port: $COM_PORT"

sleep 0.5s

BIN_PATH=$(wslpath -w -a ./target/thumbv7em-none-eabihf/release/$BIN)
echo "Using binary: $BIN_PATH"
defmt-print.exe -e $BIN_PATH -v serial --path COM$COM_PORT --dtr
