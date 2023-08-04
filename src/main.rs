#![allow(dead_code)]
mod bens;
use ben::*;
use bens::*;
pub fn main() -> Result<()> {
    Cli::prs_and_qry(new_mtr_set()?)?;
    Ok(())
}
