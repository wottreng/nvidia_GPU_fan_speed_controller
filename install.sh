#!/bin/bash

printf "\033[0;32m [*] start GPU fan controller install [*] \n \033[0m"
# download repository to temporary memory
printf "\033[0;32m [*] download repository to /tmp folder \n \033[0m"
wget -O /tmp/main.zip https://github.com/wottreng/nvidia_GPU_fan_controller/archive/refs/heads/main.zip
# unzip it
printf "\033[0;32m [*] unzip repository download \n \033[0m"
unzip /tmp/main.zip -d /tmp/
# make python file executable
printf "\033[0;32m [*] make python file executable \n \033[0m"
chmod 777 /tmp/nvidia_GPU_fan_controller-main/python/gpu_fan_control.py
# mv to system executable path
printf "\033[0;32m [*] moved gpu_fan_control.py into system /bin for execution \n \033[0m"
printf "\033[0;31m [!] root privilege needed \n \033[0m"
sudo mv /tmp/nvidia_GPU_fan_controller-main/python/gpu_fan_control.py /bin/
# set fan control to ON
printf "\033[0;32m [*] set nvidia fan control to ON \n \033[0m"
nvidia-settings -a [gpu:0]/GPUFanControlState=1
# -----
# add systemd service file
printf "\033[0;32m [*] add service file to systemd \n \033[0m"
printf "\033[0;31m [!] root privilege needed \n \033[0m"
sudo mv /tmp/nvidia_GPU_fan_controller-main/systemd/gpu_fan_control.service /etc/systemd/system/
# tell systemd to run it now and on start up
printf "\033[0;32m [*] enable service file for systemd \n \033[0m"
printf "\033[0;31m [!] root privilege needed \n \033[0m"
sudo systemctl daemon-reload
sudo systemctl enable gpu_fan_control.service
sudo systemctl start gpu_fan_control.service
# check if it is working
printf "\033[0;32m [*] lets check systemd to make sure its working... \n \033[0m"
systemctl status gpu_fan_control.service
#
printf "\033[0;32m [*] Cheers! \n \033[0m"

# NOTES :
#   if there is an issue you can stop systemd service with: sudo systemctl stop gpu_fan_control.service
#   python script logs to /tmp folder: gpu_fan_control_dd-Mmm-2022.log
#
