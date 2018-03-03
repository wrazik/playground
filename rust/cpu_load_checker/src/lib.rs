use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn remove_trailing_newline(s: &mut String) {
    let len_withoutcrlf = s.trim_right().len();
    s.truncate(len_withoutcrlf);
}

pub fn read_max_cpu_freq(core_nb: usize) -> Result<u64>{
    let max_freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", core_nb);
    let mut file = File::open(max_freq_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    remove_trailing_newline(&mut content);
    Ok(content.parse()?)
}
