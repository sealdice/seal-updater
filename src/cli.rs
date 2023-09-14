use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[clap(group(ArgGroup::new("caller-info")
    .args(["exec-path", "pid"])))]
pub struct CliArgs {
    /// Path to update file
    #[arg(long)]
    pub upgrade: String,
    /// Path to caller executable
    #[arg(long = "exec-path")]
    pub exec_path: Option<String>,
    /// Caller's PID
    #[arg(long = "pid")]
    pub pid: Option<u32>,
}