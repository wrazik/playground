use std::fs::File;
//use std::io;
use std::error;
use std::io::Read;
 
extern crate num_cpus;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn remove_trailing_newline(s: &mut String) {
    let len_withoutcrlf = s.trim_right().len();
    s.truncate(len_withoutcrlf);
}

fn read_max_cpu_freq(core_nb: usize) -> Result<u64>{
    let max_freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", core_nb);
    let mut file = File::open(max_freq_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    remove_trailing_newline(&mut content);
    Ok(content.parse()?)
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
