use psutil::process::processes;

fn main() {
    let processtable=processes().unwrap();
    let mut fulloutput:String;
    let mut output2:String;
    let mut process_listing:Vec<String> = Vec::new();
    for prc in processtable {
        let mut prc = prc.unwrap();
        let output = format!(
            "{:>6}\t{:>2.1}\t{:>2.1}\t", 
            prc.pid(), 
            prc.cpu_percent().unwrap(), 
            prc.memory_percent().unwrap());
        match prc.cmdline().unwrap() {
            Some(cmdlin) => { 
                output2 = format!("{0:.50}", cmdlin);
            }
            None => { 
                output2 = format!("[{}]", prc.name().unwrap()); 
                }
            }
        fulloutput = output + &output2; 
        process_listing.push(fulloutput.clone());
        //println!("{fulloutput}");

        for eachline in &process_listing {
            println!("{}", eachline);
            }
    }
}
