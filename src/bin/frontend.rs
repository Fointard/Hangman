use anyhow::{Context, Result};
use pendu;

fn main() -> Result<()> {
    pendu::play().with_context(|| "Error during the game")?;
    Ok(())
}
