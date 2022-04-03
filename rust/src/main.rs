use my_lib::my_functions::*;

fn main() {
    //
    log_to_ram("[*] gpu temp monitor started [*]".to_string());
    //
    let polling_timeout: u64 = 5; // seconds between readings
    // --
    let mut gpu_temp_global: u8 = 0;
    let mut fan_speed_global: u8 = 0;
    //
    let display_global = get_display();
    //
    loop {
        let gpu_temp_new: u8 = get_gpu_temp();
        if check_difference_in_temp(&mut gpu_temp_global, gpu_temp_new) {
            log_to_ram(format!("gpu_temp: {gpu_temp_global}\n"));
            let fan_speed_request = fan_curve_logarithm(gpu_temp_global);
            if fan_speed_request != fan_speed_global {
                fan_speed_global = fan_speed_request;
                log_to_ram(format!("fan speed requested: {fan_speed_global}\n"));
                let status: bool = set_gpu_fan_speed(fan_speed_global, display_global.clone());
                if status == true {
                    log_to_ram("fan set success\n".to_string());
                } else {
                    log_to_ram("[!] ERROR [!] could not assign fan speed!\n".to_string());
                }
            }
        }
        sleep_for_period(polling_timeout);
    }
}
