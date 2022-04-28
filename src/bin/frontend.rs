use anyhow::{Context, Result};

fn main() -> Result<()> {
	pendu::play().with_context(|| "Error during the game")?;
	Ok(())
}
