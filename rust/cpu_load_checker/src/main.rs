//use std::fs::File;
//use std::io::prelude::*;
use std::process::Command;
use std::str;
 
extern crate num_cpus;

fn read_max_cpu_freq(core_nb: usize) -> Result<u64, std::num::ParseIntError> {
    let max_freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", core_nb);
    let max_freq_info = Command::new("cat")
        .arg(max_freq_path)
        .output()
        .expect("Do you have cpuinfo_max_freq?"); 
    let mut cpu_freq = String::from(str::from_utf8(&max_freq_info.stdout).unwrap());
    //remove newline
    cpu_freq.pop();
    cpu_freq.parse::<u64>()
}

fn main() {
    let proc_count = num_cpus::get();
    for i in 0..proc_count {
        match read_max_cpu_freq(0) {
            Ok(max_freq) => {
                println!("CPU{} max frequency: {}\n", i, max_freq);
            }
            Err(error) => {
                println!("Error parsing max freq: {} \n", error.to_string());
            }
        }
    }
}
