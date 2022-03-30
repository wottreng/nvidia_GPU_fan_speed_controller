#!/usr/bin/env python3
'''
run with systemd to constantly monitor GPU and control fan speed
see systemd folder for service file
'''
import os
import subprocess
import time
from datetime import datetime

# how often do you want to check the GPU, cant be '0'
POLLING_TIME = 5  # seconds

# --
def get_gpu_temps(verbose: bool = False) -> int:
    gpu_data = os.popen("nvidia-smi -q -d temperature").readlines()
    gpu_temp_string = gpu_data[10].strip().replace(" ", "").split(":")[1]
    if verbose: print(f"[*] current gpu temp: {gpu_temp_string}")
    gpu_temp_int = int(gpu_temp_string.replace("C", ""))
    return gpu_temp_int


# return desired fan speed % based on temp C
def fan_curve_logarithm(gpu_temp: int) -> int:
    if gpu_temp < 20:
        return 10
    elif gpu_temp < 30:
        return 15
    elif gpu_temp < 35:
        return 20
    elif gpu_temp < 45:
        return 30
    elif gpu_temp < 55:
        return 50
    elif gpu_temp < 65:
        return 80
    else:
        return 100


# input fan speed in %
def set_gpu_fan_speed(fan_speed: int = 30):
    output = subprocess.run(["nvidia-settings", "-a", f"[fan:0]/GPUTargetFanSpeed={fan_speed}"],
                   capture_output=True,
                   text=True)
    if f"assigned value {fan_speed}" in output.stdout:
        return True
    else:
        print("[!] ERROR [!] could not assign fan speed!")
        return False


def return_gpu_information(verbose:bool=False) -> str:
    output = subprocess.check_output("nvidia-smi").decode("utf-8")
    if verbose: print(output)
    return output


# log outputs to temporary memory
def log_to_ram(data:str):
    date = datetime.now()
    time_format = date.strftime("%H:%M:%S")
    with open("/tmp/gpu_fan_control.log", "a") as file:
        file.write(f"{time_format}: {data}\n")


if __name__ == '__main__':
    log_to_ram("[*] gpu fan control started [*]")
    fan_speed = 100
    gpu_temp = 100
    while True:
        # --
        gpu_temp_new = get_gpu_temps()
        if gpu_temp_new != gpu_temp:
            gpu_temp = gpu_temp_new
            log_to_ram(f"gpu_temp: {gpu_temp}")
        # --
        fan_speed_new = fan_curve_logarithm(gpu_temp)
        if fan_speed_new != fan_speed:
            fan_speed = fan_speed_new
            log_to_ram(f"fan speed requested: {fan_speed}%")
            set_gpu_fan_speed(fan_speed)
        # --
        time.sleep(POLLING_TIME)

