mod bens;
use anyhow::Result;
use bens::*;
pub fn main() -> Result<()> {
    run_mtr_qrys()?;
    Ok(())
}
