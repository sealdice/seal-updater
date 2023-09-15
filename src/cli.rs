use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Path to update file
    #[arg(long)]
    pub upgrade: String,
    /// Caller's PID
    #[arg(long = "pid")]
    pub pid: u32,
}