Fork of https://github.com/guyo13/mt-interface/tree/main

Goal is to be able to control and read zigbee devices through a Rust API without
[zigbee2mqtt](www.zigbee2mqtt.io) while [zigbee2mqtt](www.zigbee2mqtt.io) is
used for managing the zigbee network (updating/adding/removing/naming etc). 

This is tested and only supports the `CC2652P USB Dongle` (Sonoff zigbee 3.0 USB dongle
plus

# Acknowledgements

This would have been quite hard if I could read the `zigbee-herdsman` and
`zigpy-znp` code. Many thanks to their authors and contributors.

# License

The code in `ts_fragment` files belongs to 'Koen Kanters' and was auto translated
from zigbee herdsman https://github.com/Koenkk/zigbee-herdsman
