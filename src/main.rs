use clap::Parser;
use crate::cli::CliArgs;

mod cli;
#[path= "runner/lib.rs"]
mod lib;

fn main() {
    //let args = CliArgs::parse();
    let args = CliArgs {
        upgrade: String::new(),
        pid: None
    };
    if let Err(err) = lib::run_upgrade(&args) {
        println!("\x1b[31m出现错误：{}\x1b[0m", err);
    }
}
