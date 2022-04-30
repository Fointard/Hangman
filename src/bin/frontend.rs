use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[clap(
	author = "Fointard",
	version = "0.1.0",
	about = "A simple hangman game written in Rust."
)]
struct Args {
	#[clap(short, long, default_value = "library.txt")]
	library: String,
}

fn main() -> Result<()> {
	let args = Args::parse();

	hangman::play(args.library).with_context(|| "Error during the game")?;
	Ok(())
}
