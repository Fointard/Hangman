use anyhow::{Context, Result};

fn main() -> Result<()> {
	hangman::play().with_context(|| "Error during the game")?;
	Ok(())
}
