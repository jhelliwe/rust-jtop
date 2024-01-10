extern crate cpu_monitor;
use cpu_monitor::CpuInstant;
use std::io;
use std::process;
use std::time::Duration;

pub mod linuxproc;
pub mod render;
pub mod terminfo;

fn main() -> Result<(), io::Error> {
    loop {
        // terminfo - terminal size calculations here
        let (session_width, session_height) = terminfo::termsize(); // termsize() exits if it could
                                                                    // not detect the terminal size
        let usable_width = session_width - 25; // The numerical fields length is subtracted so that
                                               // we know where to truncate the cmdline
        let usable_height = session_height - 7; // And we also subtract the top headers so we know
                                                // how much room there is left for the process
                                                // listing
                                                // process_listing is a Vector of all running processes
        let mut process_listing: Vec<linuxproc::ProcessInstance> = Vec::new();

        // calculate CPU usage and generate a percentage bar
        let start = CpuInstant::now()?;
        
        // Grab a "ps -ef" and store it in a Vector
        linuxproc::get_process_list(&mut process_listing, usable_width);

        std::thread::sleep(Duration::from_millis(2000)); // Wait 2 seconds and take another
                                                         // measurement
        let end = CpuInstant::now()?;
        let duration = end - start;
        let cpuperc = duration.non_idle() * 100.;
        let cpubar = render::drawbar("CPU%", session_width, cpuperc);

        // calculate memory usage and generate a percentage bar
        let memresult: linuxproc::MemoryResult = linuxproc::memory_proc(); // memory_proc reads
                                                                           // /proc and returns a
                                                                           // Result<T, E>
        match memresult {
            Ok(linuxproc::MemoryStat {
                total: memtotal,
                used: memused,
            }) => {
                let memperc = (memused as f64 / memtotal as f64) as f64 * 100.0;
                let membar = render::drawbar("MEM%", session_width, memperc as f64);

                render::screen(
                    &mut process_listing,
                    usable_width,
                    usable_height,
                    &cpubar,
                    &membar,
                );
            }
            Err(_) => {
                eprintln!("Problem reading /proc");
                process::exit(1);
            }
        }
    }
}
