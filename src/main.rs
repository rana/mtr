mod ben;
mod bens;
use crate::ben::*;
use bens::*;
use anyhow::{bail, Result};
use clap::{arg, Parser};
pub fn main() -> Result<()> {
    let cli = Cli::parse();
    if let Err(e) = DBG.set(cli.dbg) {
        bail!(e);
    }
    run_mtr_qrys()?;
    Ok(())
}
/// Benchmark, query, and analyze functions
#[derive(Parser, Debug)]
pub struct Cli {
    /// Print debug information
    #[arg(short = 'd', long)]
    dbg: bool,
}
