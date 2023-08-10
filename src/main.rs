mod ben;
mod bens;
mod itr;
use anyhow::{bail, Result};
use bens::*;
use clap::{arg, Parser};
use crate::ben::*;
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
