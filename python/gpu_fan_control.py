#!/usr/bin/env python3
'''
run with systemd to constantly monitor GPU and control fan speed
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
    if gpu_temp < 30:
        return 15
    elif gpu_temp < 40:
        return 40
    elif gpu_temp < 60:
        return 65
    elif gpu_temp < 70:
        return 80
    elif gpu_temp < 80:
        return 90
    else:
        return 100


# input fan speed in %
def set_gpu_fan_speed(_display:str, fan_speed: int = 30,):
    output = os.popen(
        f"DISPLAY=:{_display.split(':')[1]} XAUTHORITY={_display} nvidia-settings -a [fan:0]/GPUTargetFanSpeed={fan_speed}").read()
    if f"assigned value {fan_speed}" in output:
        log_to_ram("fan set success")
        return True
    else:
        log_to_ram(output)
        log_to_ram("[!] ERROR [!] could not assign fan speed!")
        # print("[!] ERROR [!] could not assign fan speed!")
        return False

def return_gpu_information(verbose:bool=False) -> str:
    output = subprocess.check_output("nvidia-smi").decode("utf-8")
    if verbose: print(output)
    return output

def get_display()->str:
    output = os.popen("ps a | grep X").readlines()
    _display = output[0].split("-auth ")[1].split(" ")[0]
    return _display

# log outputs to temporary memory
def log_to_ram(data:str):
    date = datetime.now()
    time_format = date.strftime("%H:%M:%S")
    date_format = date.strftime("%d-%b-%Y")  # new log file per day
    with open(f"/tmp/gpu_fan_control_{date_format}.log", "a") as file:
        file.write(f"{time_format}: {data}\n")

# stop fan flutter by temp toggling up and down between control steps
def check_difference_in_temp(old_temp:int,new_temp:int)->bool:
    DIFF = 3
    if abs(new_temp-old_temp) > DIFF:
        return True
    else:
        return False

if __name__ == '__main__':
    log_to_ram("[*] gpu fan control started [*]")
    # get X server
    display_global = get_display()
    #
    fan_speed = 100
    gpu_temp = 100
    while True:
        # --
        gpu_temp_new = get_gpu_temps()
        if check_difference_in_temp(gpu_temp, gpu_temp_new):
            gpu_temp = gpu_temp_new
            log_to_ram(f"gpu_temp: {gpu_temp}")
            # --
            fan_speed_new = fan_curve_logarithm(gpu_temp)
            if fan_speed_new != fan_speed:
                fan_speed = fan_speed_new
                log_to_ram(f"fan speed requested: {fan_speed}%")
                set_gpu_fan_speed(display_global, fan_speed)
        # --
        time.sleep(POLLING_TIME)


