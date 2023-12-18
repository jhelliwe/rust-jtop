use psutil::process::processes;
use std::fmt;
use std::thread;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};

extern crate cpu_monitor;
use std::io;
use cpu_monitor::CpuInstant;

struct ProcessInstance {
    pid: u32,
    cpu_percent: f32,
    memory_percent: f32,
    commandline: String,
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

fn main() -> Result<(), io::Error> {
    loop {
        let mut session_width: usize = 0;
        let mut session_height: usize = 0;
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            session_width = w as usize;
            session_height = h as usize;
        } else {
            println!("Unable to get terminal size");
        }
        session_width -= 25;
        session_height -= 7;

        // full_process_table contains details of all running processes
        let full_process_table = processes().unwrap();
        let mut process_listing: Vec<ProcessInstance> = Vec::new();

        // iterate through the full process table
        for individual_process_wrapped in full_process_table {
            match individual_process_wrapped {
                Ok(mut individual_process) => match individual_process.cmdline() {
                    Ok(Some(output)) => {
                        let outfmt = format!("{0:.session_width$}", output);
                        let process_info_to_push = ProcessInstance {
                            pid: individual_process.pid(),
                            cpu_percent: match individual_process.cpu_percent() {
                                Ok(cpu_percent) => { if output.contains("jtop" ) { 0.0 } else { cpu_percent } },
                                Err(_) => { 0.0 } 
                            },
                            memory_percent: match individual_process.memory_percent() {
                                Ok(memory_percent) => { memory_percent },
                                Err(_) => { 0.0 }
                            },
                            commandline: outfmt,
                        };
                        process_listing.push(process_info_to_push);
                    }
                    Ok(None) => {
                        // let outfmt = format!("[{0:.session_width$}]", individual_process.name().unwrap());
                        let outfmt = format!("[{0:.session_width$}]", match individual_process.name() {
                            Ok(name) => { name },
                            Err(_) => { "[defunct]".to_string() }
                        });
                        let process_info_to_push = ProcessInstance {
                            pid: individual_process.pid(),
                            cpu_percent: match individual_process.cpu_percent() {
                                Ok(cpu_percent) => { cpu_percent },
                                Err(_) => { 0.0 } 
                            },
                            memory_percent: match individual_process.memory_percent() {
                                Ok(memory_percent) => { memory_percent },
                                Err(_) => { 0.0 }
                            },
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

        let start = CpuInstant::now()?;
        std::thread::sleep(Duration::from_millis(2000));
        let end = CpuInstant::now()?;
        let duration = end - start;
        let cpuperc = duration.non_idle() * 100.;
        let bar = (session_width + 20 ) as f64 * (cpuperc / 100.);
        let mut barcounter = 0.;
        print!("CPU% {:.0}", cpuperc);
        while barcounter <= bar { 
            print!("*");
            barcounter += 1.;
        }
        println!();


        // temporary dump to stdout
        process_listing.sort_by(|a, b| b.cpu_percent.total_cmp(&a.cpu_percent));
        let n_vec_element = process_listing.len();
        println!("jtop!");
        println!("nproc {}", n_vec_element);
        println!();
        println!("{:>6}\t{:>2}\t{:>2}\t{}", "PID", "CPU%", "MEM%", "CMDLINE");
        let mut count = 0;
        while count <= session_height && count <= n_vec_element {
            println!("{}", process_listing[count]);
            count += 1;
        }
        //thread::sleep(Duration::from_millis(2000))
    }
    // Ok(())
}
