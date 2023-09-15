use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Path to update file
    #[arg(long)]
    pub upgrade: String,
    /// Caller's PID
    #[arg(long = "pid")]
    pub pid: Option<u32>,
    #[arg(long, default_value_t = String::new())]
    pub dest: String
}