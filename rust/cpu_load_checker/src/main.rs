extern crate cpu_load;
use std::io::Write;
use cpu_load::get_curr_freqs;

fn main() {
    loop {
        print!("\r");
        match get_curr_freqs() {
            Ok(freq_vec) => {
                for (core_nb, freq) in freq_vec {
                    print!("CPU{}: {:.2}% \t", core_nb, freq);
                }
            },
            Err(err) => {
                println!("Error: {}\n", err.to_string());
            }
        }
        std::io::stdout().flush().unwrap();
    }
}
