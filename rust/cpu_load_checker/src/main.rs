extern crate num_cpus;
extern crate cpu_load;

use cpu_load::read_max_cpu_freq;

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
