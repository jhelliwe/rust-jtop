use crate::linuxproc;
use ansi_term::{Colour, Style};
use clearscreen;

pub fn screen(
    process_listing: &mut Vec<linuxproc::ProcessInstance>,
    mut _usable_width: usize,
    usable_height: usize,
    cpubar: &String,
    membar: &String,
    swapbar: &String,
) {
    _usable_width += 25;
    clearscreen::clear().unwrap();
    process_listing.sort_by(|a, b| b.cpu_percent.total_cmp(&a.cpu_percent));
    let n_vec_element = process_listing.len();
    println!("{}", cpubar);
    println!("{}", membar);
    println!("{}", swapbar);
    print!("{}", Colour::Green.paint("jtop! "));
    println!("nproc {}", n_vec_element);
    println!();
    let style = Style::new().reverse();
    print!("{}", style.paint("PID     CPU%    MEM%    CMDLINE"));
    for _filler in 31.._usable_width {
        print!("{}", style.paint(" "));
    }
    println!();
    let mut count = 0;
    while count <= usable_height && count <= n_vec_element {
        println!("{}", process_listing[count]);
        count += 1;
    }
}

pub fn drawbar(title: &str, width: usize, percent: f64) -> String {
    let mut _rendered_bar = String::new();
    let bar = (width - 11) as f64 * percent / 100.;
    let mut barcounter = 0.;
    _rendered_bar = format!("{} {:2.0}% ", title, percent);
    while barcounter <= bar {
        //_rendered_bar = [&_rendered_bar, "|"].concat();
        if percent <= 50.0 {
            _rendered_bar = format!("{}{}", _rendered_bar, Colour::Green.paint("|"));
        } else if percent <= 75.0 {
            _rendered_bar = format!("{}{}", _rendered_bar, Colour::Yellow.paint("|"));
        } else {
            _rendered_bar = format!("{}{}", _rendered_bar, Colour::Red.paint("|"));
        }
        barcounter += 1.;
    }
    return format!("{}", _rendered_bar);
}
