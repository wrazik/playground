use std::error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::vec::Vec;
use std::{thread, time};

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn is_cpu_core_line(s: &str) -> bool {
    let first_word = s.split_whitespace().next().unwrap();
    first_word.starts_with("cpu") && first_word.get(3..).unwrap().parse::<i32>().is_ok()
}

#[derive(PartialEq, Eq, Debug)]
struct ProcStats {
    core_nb: usize,
    user: i64,
    nice: i64,
    system: i64,
    idle: i64,
    iowait: i64,
    irq: i64,
    softirq: i64,
    steal: i64,
    guest: i64,
    guest_nice: i64,
}

impl ProcStats {
    fn get_idle(&self) -> i64 {
        self.idle + self.iowait
    }

    fn get_non_idle(&self) -> i64 {
        self.user + self.nice + self.system + self.irq + self.softirq + self.steal
    }

    fn get_total(&self) -> i64 {
        self.get_idle() + self.get_non_idle()
    }
}

fn get_core_nb(word: &str) -> usize {
    word.get(3..).unwrap().parse::<usize>().unwrap()
}

fn create_single_proc_stats(line: &str) -> ProcStats {
    let mut word_it = line.split_whitespace();
    let core_nb = get_core_nb(&word_it.next().unwrap());
    let as_nums: Vec<_> = word_it.map(|word| {
        word.parse::<i64>().unwrap()
    }).collect();

    ProcStats {
        core_nb: core_nb,
        user: as_nums[0],
        nice: as_nums[1],
        system: as_nums[2],
        idle: as_nums[3],
        iowait: as_nums[4],
        irq: as_nums[5],
        softirq: as_nums[6],
        steal: as_nums[7],
        guest: as_nums[8],
        guest_nice: as_nums[9],
    }
}

fn read_proc_stats() -> Result<Vec<ProcStats>> {
    let proc_stat_file = File::open("/proc/stat")?;
    let proc_stat_lines = BufReader::new(&proc_stat_file);
    let result: Vec<_> = proc_stat_lines.lines()
        .map(|line| line.unwrap())
        .filter(|line| is_cpu_core_line(&line))
        .map(|line| create_single_proc_stats(&line))
        .collect();
    Ok(result)
}

fn get_cpu_freqs(prev: &Vec<ProcStats>, curr: &Vec<ProcStats>) -> Result<Vec<(usize, f64)>> {
    assert_eq!(prev.len(), curr.len());
    let mut result  = Vec::new();

    for (previous, current) in prev.iter().zip(curr.iter()) {
        assert_eq!(previous.core_nb, current.core_nb);
        let totald = (current.get_total() - previous.get_total()) as f64;
        let idled = (current.get_idle() - previous.get_idle()) as f64;
        result.push((previous.core_nb, (totald-idled)/totald*100.0));
    }
    Ok(result)
}

pub fn get_curr_freqs() -> Result<Vec<(usize, f64)>> {
    let first_measurement = read_proc_stats()?;
    thread::sleep(time::Duration::from_secs(1));
    let second_measurement = read_proc_stats()?;
    get_cpu_freqs(&first_measurement, &second_measurement)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_cpu_core_line_test() {
        assert_eq!(false, is_cpu_core_line("fake_line"));
        assert_eq!(false, is_cpu_core_line("cpufake"));
        assert_eq!(false, is_cpu_core_line("cpu 12 31 231"));
        assert_eq!(true, is_cpu_core_line("cpu0 2 3 1 24 4 12 4"));
    }

    #[test]
    fn create_single_proc_stats_test() {
        let expected = ProcStats {
            core_nb: 3,
            user: 24082,
            nice: 250,
            system: 3356,
            idle: 65772,
            iowait: 481,
            irq: 488,
            softirq: 199,
            steal: 0,
            guest: 0,
            guest_nice: 0,
        };

        let actual = create_single_proc_stats("cpu3 24082 250 3356 65772 481 488 199 0 0 0");
        assert_eq!( actual, expected);
    }
}
