# nvidia_GPU_fan_controller
monitor GPU temps and control GPU fan

## how to make it work
* add `gpu_fan_control.py` to your `/bin/` 
* make it excutable: `sudo chmod 770 /bin/gpu_fan_control.py`
* add `gpu_fan_control.service` to `/etc/systemd/system/`
* reload systemd services: `sudo systemctl daemon-reload`
* enable fan control service: `sudo systemctl enable gpu_fan_control.service`
* start fan control service: `sudo systemctl start gpu_fan_control.service`
* check log file: `cat /tmp/gpu_fan_control.log`


NOTES: you NEED to enable manual fan control in your NVIDIA settings: `nvidia-settings` then Thermal Settings then check Enable GPU Fan Settings

Cheers, Mark
