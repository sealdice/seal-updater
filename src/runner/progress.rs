use std::cmp::max;
use std::error::Error;
use std::io::Write;

pub struct ProgressBar {
    max: usize,
    current: usize,
    padding: usize,
}

impl ProgressBar {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            current: 0,
            padding: format!("{max}").len() * 2 + 9,
        }
    }

    fn get_width(&self) -> usize {
        let term_width = match term_size::dimensions_stdout() {
            None => 80,
            Some((w, _)) => w,
        };

        let mut bar_width = term_width - (self.padding + 2);
        bar_width = max(bar_width, self.padding + 20);
        bar_width / 10 * 10
    }

    pub fn blackout(&self) {
        let width = match term_size::dimensions_stdout() {
            None => 80,
            Some((w, _)) => w,
        };
        print!("\r{}\r", " ".repeat(width));
        _ = std::io::stdout().flush();
    }

    pub fn progress(&mut self) {
        _ = self.progress_by(1);
    }

    pub fn progress_by(&mut self, offset: i32) -> Result<(), Box<dyn Error>> {
        let new_progress = self.current as i32 + offset;
        if new_progress > self.max as i32 {
            Err(format!(
                "Overflow: {} is larger than maximum value {}",
                new_progress, self.max
            ))?;
        }
        if new_progress < 0 {
            Err(format!(
                "Underflow: {} with {} results in negative value",
                self.current, offset
            ))?;
        }

        let width = self.get_width();
        let percent = new_progress as f64 / self.max as f64;
        let fill = (width as f64 * percent) as usize;
        let empty = width - fill;

        let output = format!(
            "\r[{}{}] {}% ({}/{})",
            "#".repeat(fill),
            " ".repeat(empty),
            (percent * 100.0) as i32,
            new_progress,
            self.max
        );
        print!("{output}");
        _ = std::io::stdout().flush();

        if new_progress >= self.max as i32 {
            println!();
        } else {
            self.current = new_progress as usize;
        }

        Ok(())
    }
}
