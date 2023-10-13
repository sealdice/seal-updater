use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArgs {
    /// Path to update file
    #[arg(long)]
    pub upgrade: String,
    /// Caller's PID
    #[arg(long = "pid")]
    pub pid: u32,
    /// Current working directory
    #[arg(long, default_value_t = String::from("./"))]
    pub cwd: String,
    /// Print more info
    #[arg(long)]
    pub verbose: bool,
    /// Do not start SealDice after decompressing
    #[arg(long = "skip-startup")]
    pub skip_startup: bool,
}
