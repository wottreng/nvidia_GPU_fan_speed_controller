# nvidia_GPU_fan_controller
monitor GPU temps and control GPU fan

## how to make it work
* add `gpu_fan_control.py` to your `/bin/` 
* make it excutable: `sudo chmod 777 /bin/gpu_fan_control.py`
* add `gpu_fan_control.service` to `/etc/systemd/system/`
* reload systemd services: `sudo systemctl daemon-reload`
* enable fan control service: `sudo systemctl enable gpu_fan_control.service`
* start fan control service: `sudo systemctl start gpu_fan_control.service`
* check log file in `/tmp` folder


NOTES: 
* you NEED to enable manual fan control in your NVIDIA settings: `nvidia-settings` then Thermal Settings then check Enable GPU Fan Settings
* tested on Linux Mint 20.3 Cinnamon, kernel: 5.13, nvidia driver: 470.103.01

Cheers, Mark
