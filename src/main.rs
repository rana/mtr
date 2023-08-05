mod ben;
mod bens;
use crate::ben::*;
use bens::*;
pub fn main() -> Result<()> {
    Cli::prs_and_qry(new_mtr_set()?)?;
    Ok(())
}
