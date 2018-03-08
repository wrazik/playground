extern crate cpu_load;
use cpu_load::get_curr_freqs;

fn main() {
    match get_curr_freqs() {
        Ok(freq_vec) => {
            for (core_nb, cpu_freq) in freq_vec {
                print!("| Cpu{}: {:.2}% ", core_nb, cpu_freq);
            }
            println!("|");
        }
        Err(err) => {
            println!("Error: {}\n", err.to_string());
        }
    }
}
