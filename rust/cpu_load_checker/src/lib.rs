use std::error;
#[cfg(not(test))]
use std::fs::File;
#[cfg(not(test))]
use std::io::Read;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn remove_trailing_newline(s: &mut String) {
    let len_withoutcrlf = s.trim_right().len();
    s.truncate(len_withoutcrlf);
}

trait FileOp {
    fn get_file_content(&self, filename: String) -> Result<String>;
}

struct Fs;

#[cfg(not(test))]
impl FileOp for Fs {
    fn get_file_content(&self, filename: String) -> Result<String> {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

pub fn read_max_freq(core_nb: usize) -> Result<u64>{
    let filename = format!("/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq", core_nb);
    let mut content = Fs.get_file_content(filename)?;
    remove_trailing_newline(&mut content);
    Ok(content.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    impl FileOp for Fs {
        fn get_file_content(&self, _filename: String) -> Result<String> {
            Ok(String::from(Fs::CONTENT))
        }
    }

    #[test]
    fn remove_trailing_newline_test() {
        let test_str = "Test";
        let mut windows_specific = format!("{}\r\n", test_str);
        let mut linux_specific = format!("{}\n", test_str);
        let mut no_endline = format!("{}", test_str);
        remove_trailing_newline(&mut windows_specific);
        remove_trailing_newline(&mut linux_specific);
        remove_trailing_newline(&mut no_endline);
        assert_eq!(windows_specific, test_str);
        assert_eq!(linux_specific, test_str);
        assert_eq!(no_endline, test_str);
    }

    #[test]
    fn read_max_freq_test() {
        const FREQ: u64 = 102303;

        impl Fs {
             const CONTENT: &'static str = "102303\n";
        }

        assert_eq!(read_max_freq(0).unwrap(), FREQ);
    }
}
