extern crate cpu_load;
use cpu_load::get_curr_freqs;

fn main() {
    match get_curr_freqs() {
        Ok(freq_vec) => {
            for (core_nb, val) in freq_vec {
                print!("|\tCPU{}: {:.2}%\t", core_nb, val);
            }
            println!("|");
        }
        Err(err) => {
            println!("Error: {}\n", err.to_string());
        }
    }
}
