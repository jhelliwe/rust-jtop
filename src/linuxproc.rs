use psutil::process::processes;
use std::error::Error;
use std::{fmt, fs};

pub struct ProcessInstance {
    pub pid: u32,
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub commandline: String,
}

impl fmt::Display for ProcessInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>6}\t{:>2.1}\t{:>2.1}\t{}",
            self.pid, self.cpu_percent, self.memory_percent, self.commandline
        )
    }
}

pub struct MemoryStat {
    pub total: usize,
    pub used: usize,
    pub swaptotal: usize,
    pub swapfree: usize
}

impl MemoryStat {
    pub fn total(&self) -> usize {
        self.total
    }

    pub fn used(&self) -> usize {
        self.used
    }
}

pub fn get_process_list(process_listing: &mut Vec<ProcessInstance>, usable_width: usize) {
    let full_process_table = processes().unwrap();
    // iterate through the full process table
    for individual_process_wrapped in full_process_table {
        match individual_process_wrapped {
            Ok(mut individual_process) => match individual_process.cmdline() {
                Ok(Some(output)) => {
                    let outfmt = format!("{0:.usable_width$}", output);
                    let process_info_to_push = ProcessInstance {
                        pid: individual_process.pid(),
                        cpu_percent: match individual_process.cpu_percent() {
                            Ok(cpu_percent) => {
                                if output.contains("jtop") {
                                    0.0
                                } else {
                                    cpu_percent
                                }
                            }
                            Err(_) => 0.0,
                        },
                        memory_percent: individual_process.memory_percent().unwrap_or(0.0),
                        commandline: outfmt,
                    };
                    process_listing.push(process_info_to_push);
                }
                Ok(None) => {
                    let outfmt = format!(
                        "[{0:.usable_width$}]",
                        match individual_process.name() {
                            Ok(name) => {
                                name
                            }
                            Err(_) => {
                                "[defunct]".to_string()
                            }
                        }
                    );
                    let process_info_to_push = ProcessInstance {
                        pid: individual_process.pid(),
                        cpu_percent: individual_process.cpu_percent().unwrap_or(0.0),
                        memory_percent: individual_process.memory_percent().unwrap_or(0.0),
                        commandline: outfmt,
                    };
                    process_listing.push(process_info_to_push);
                }
                Err(_) => {
                    continue;
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
}

pub type MemoryResult = Result<MemoryStat, Box<dyn Error>>;
pub fn memory_proc() -> MemoryResult {
    let mut return_value = MemoryStat { total: 0, used: 0, swaptotal: 0, swapfree: 0 };
    let mr = fs::read_to_string("/proc/meminfo");
    match mr {
        Ok(contents) => {
            for eachline in contents.lines() {
                if eachline.contains("MemTotal:") {
                    let memvec: Vec<&str> = eachline.split_whitespace().collect();
                    let memtotal = &memvec[1];
                    return_value.total = memtotal.parse().unwrap();
                } else if eachline.contains("MemAvailable:") {
                    let memvec: Vec<&str> = eachline.split_whitespace().collect();
                    let memavail = &memvec[1];
                    return_value.used = memavail.parse::<usize>().unwrap();
                } else if eachline.contains("SwapTotal:") {
                    let memvec: Vec<&str> = eachline.split_whitespace().collect();
                    let memtotal = &memvec[1];
                    return_value.swaptotal = memtotal.parse().unwrap();
                } else if eachline.contains("SwapFree:") {
                    let memvec: Vec<&str> = eachline.split_whitespace().collect();
                    let memtotal = &memvec[1];
                    return_value.swapfree = memtotal.parse().unwrap();
                }
            }
        }
        Err(err) => {
            return Err(Box::new(err));
        }
    }
    Ok(return_value)
}
