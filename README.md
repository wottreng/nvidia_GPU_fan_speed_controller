# nvidia_GPU_fan_controller
monitor GPU temps and control GPU fan

## TLDR cli magic:
copy and paste into your cmd line for installation
```markdown
curl https://raw.githubusercontent.com/wottreng/nvidia_GPU_fan_controller/main/install.sh | bash
```

## how to make it work (manual install)
NOTE: see `install.sh` for all commmands needed! 
* add `gpu_fan_control.py` to your `/bin/` 
* make it excutable: `sudo chmod 777 /bin/gpu_fan_control.py`
* turn nvidia fan control ON: `nvidia-settings -a [gpu:0]/GPUFanControlState=1`
* add `gpu_fan_control.service` to `/etc/systemd/system/`
* reload systemd services: `sudo systemctl daemon-reload`
* enable fan control service: `sudo systemctl enable gpu_fan_control.service`
* start fan control service: `sudo systemctl start gpu_fan_control.service`
* check log file in `/tmp` folder

## Optional Configuration
you can modify temp to fan speed curve in the `fan_curve_algorithm` function

## NOTES: 
* you NEED to enable manual fan control in your NVIDIA settings: 
  * GUI: `nvidia-settings` then Thermal Settings then check Enable GPU Fan Settings
  * CLI: `nvidia-settings -a [gpu:0]/GPUFanControlState=1`
* tested on Linux Mint 20.3 Cinnamon, kernel: 5.13, nvidia driver: 470.103.01
* rust version added if you rather go that route

Cheers, Mark

<a href=" https://www.buymeacoffee.com/wottreng" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/default-orange.png" alt="Buy Me A Coffee" height="41" width="174"></a>
