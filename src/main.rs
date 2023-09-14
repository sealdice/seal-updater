use clap::Parser;
use crate::cli::CliArgs;

mod cli;
#[path= "runner/lib.rs"]
mod lib;

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = lib::run_upgrade(&args) {
        eprintln!("\033[31m出现错误：{}\033[0m", err);
    }
}
