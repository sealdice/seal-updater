use std::io;
use std::io::Write;

pub fn print_progress(prog: usize, total: usize) {
    let percentage = (prog as f64 / total as f64) * 100.0;
    let last = if prog <= 1 {
        String::new()
    } else {
        format!("{:.1}%", ((prog - 1) as f64 / total as f64) * 100.0)
    };

    print!("{}", "\x08".repeat(last.len()));
    io::stdout().flush().unwrap();
    print!("{:.1}%", percentage);
    io::stdout().flush().unwrap();
}