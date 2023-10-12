use crate::cli::CliArgs;
use clap::Parser;
use once_cell::sync::Lazy;

#[cfg(target_family = "windows")]
pub static SEAL_EXE: &str = "sealdice-core.exe";
#[cfg(target_family = "unix")]
pub static SEAL_EXE: &str = "sealdice-core";
#[cfg(target_family = "windows")]
pub static UPD_NAME: &str = "seal-updater.exe";
#[cfg(target_family = "unix")]
pub static UPD_NAME: &str = "seal-updater";

pub static CMD_OPT: Lazy<CliArgs> = Lazy::new(CliArgs::parse);
