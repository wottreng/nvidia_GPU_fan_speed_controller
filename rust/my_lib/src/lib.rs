/*
 supporting functions for gpu fan controller
 written by Mark Wottreng
 */

pub mod my_functions {

    // return X display
    pub fn get_display() -> String {
        let debug:bool=false;
        let output = os_cmd("ps a | grep X".to_string());
        let _display_vector_0: Vec<&str> = output.split("\n").collect();
        let _display_vector_1: Vec<&str> = _display_vector_0[0].split("-auth ").collect();
        let _display_vector_2: Vec<&str> = _display_vector_1[1].split(" ").collect();
        if debug{println!("{:?}", _display_vector_2[0])};
        return _display_vector_2[0].to_string();
    }

    //
    pub fn sleep_for_period(period: u64) {
        use std::{thread,time};
        thread::sleep(time::Duration::from_secs(period));
    }

    // stop fan flutter by temp toggling up and down between control steps
    pub fn check_difference_in_temp(old_temp: &mut u8, new_temp: u8) -> bool {
        let difference: u8 = 4;
        let absolute_diff = old_temp.clone() - new_temp;
        return if absolute_diff > difference {
            *old_temp = new_temp;
            true
        } else { false };
    }

    // --
    pub fn get_gpu_temp() -> u8 {
        let verbose: bool = false;
        let mut gpu_temp: u8 = 0;
        let output_str = os_cmd("nvidia-smi -q -d temperature".to_string());
        //
        let index_string = "GPU Current Temp                  : ";
        if let Some(result_i) = output_str.find(index_string) {
            //println!("{:?}",result_i);
            let len = index_string.chars().count();
            if let Some(inner) = output_str.get(result_i + len..result_i + len + 2) {
                //println!("{:?}",inner);
                gpu_temp = inner.parse().unwrap();
            }
        }
        //
        if verbose { println!("gpu temp: {:?}", gpu_temp); };
        return gpu_temp;
    }

    //
    pub fn fan_curve_logarithm(gpu_temp: u8) -> u8 {
        return if gpu_temp < 30 {
            15
        } else if gpu_temp < 40 {
            40
        } else if gpu_temp < 60 {
            65
        } else if gpu_temp < 70 {
            80
        } else if gpu_temp < 80 {
            90
        } else {
            100
        };
    }

    //
    pub fn set_gpu_fan_speed(fan_speed: u8, _display: String) -> bool {
        let verbose:bool = false;
        //
        let cmd_str = format!("DISPLAY=:0 XAUTHORITY={_display} nvidia-settings -a [fan:0]/GPUTargetFanSpeed={fan_speed}");
        let output_str = os_cmd(cmd_str);
        if verbose{println!("{:?}", output_str)};
        let success_str = format!("assigned value {}", fan_speed);
        return if output_str.contains(&success_str) {
            true
        } else {
            false
        };
    }

    //
    pub fn read_string_from_file(path: String, filename: String) -> Result<String, std::io::Error> {
        let verbose: bool = false;
        //
        let file_path = format!("{:?}/{:?}", path, filename).replace(&['\"'], "");
        if verbose {
            println!("read_string_from_file verbose:");
            println!("filename: {:?}", filename);
            println!("path: {:#?}", &path);
            println!("file_path: {:#?}", file_path);
            println!("----------");
        }
        // read file ---
        use std::fs;
        let contents = fs::read_to_string(&file_path)?;
        //
        Ok(contents)
    }

    //
    pub fn write_string_to_file(data: String, path: String, filename: String) -> bool {
        use std::fs;
        //
        let verbose: bool = false;
        // clean up strings and options ---
        let file_path = format!("{:?}/{:?}", path, filename).replace(&['\"'], "");
        if verbose {
            println!("filename: {:?}", filename);
            println!("path: {:#?}", &path);
            println!("file_path: {:#?}", file_path);
        }
        // write to file --
        fs::write(&file_path, &data).unwrap();
        //
        return true;
    }

    //
    fn append_to_file(data_in: String, path: String, filename: String) -> Result<bool, std::io::Error>{
        let verbose:bool = false;
        //
        let file_contents:String;
        match read_string_from_file(path.clone(), filename.clone()){
            Ok(file_data) => {
                file_contents = file_data;
            },
            Err(e) => {
                if verbose{ println!("Error: {:?}",e)};
                file_contents = "".to_string();
            }
        };
        //
        let new_data:String = format!("{}{}", file_contents, data_in);
        if verbose{
            println!("content: {:?}", file_contents);
            println!("data: {:?}",data_in);
            println!("new_data: {:?}",new_data);
        };
        write_string_to_file(new_data, path.clone(), filename.clone());
        Ok(true)
    }

    // log output to /tmp folder
    pub fn log_to_ram(data: String) {
        //
        let date_time = chrono::offset::Local::now().to_string();
        let date_time_vec: Vec<&str> = date_time.split(" ").collect();
        let date = date_time_vec[0];
        let time_vec: Vec<&str> = date_time_vec[1].split(".").collect();
        let time = time_vec[0];
        //
        let log_name = "gpu_fan_control".to_string();
        let path = "/tmp".to_string();
        let data_format = format!("{:?}: {:?}\n",time, data.replace("\n", "")).replace("\"",""); // \n
        let filename_format = format!("{log_name}-{date}.log");
        append_to_file(data_format, path, filename_format).unwrap();
    }

    //
    fn os_cmd(cmd: String) -> String {
        let verbose: bool = false;
        //
        use std::process::Command;
        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process");
        //
        if verbose {
            println!("status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        assert!(output.status.success());
        let std_out = String::from_utf8_lossy(&output.stdout).to_string();
        return std_out;
    }
}
