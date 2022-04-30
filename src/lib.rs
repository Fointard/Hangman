mod score;

use anyhow::{Context, Result};
use rand::Rng;
use std::{
	self,
	fmt::Display,
	fs::File,
	io::{self, prelude::*, BufReader},
	path::Path,
	str,
};

pub fn play(library: String) -> Result<()> {
	let library = get_lib(library)?;
	let mut guess = String::new();
	let mut score = score::Score::new();
	let mut word_guess;
	let mut tries;

	'game: loop {
		tries = 10;
		let word = get_word(&library);
		word_guess = str::repeat("_", word.len());

		if loop {
			ask_user_input(&word_guess, &mut guess)?;

			if guess.starts_with("quit") {
				break 'game Ok(());
			}

			// Word-based guessing
			if game_is_won(word, &guess) {
				break true;
			}

			analyse_user_input(&mut guess, word, &mut word_guess, &mut tries);

			// Char-based guessing
			if game_is_won(word, &word_guess) {
				break true;
			}

			if game_is_lost(&tries) {
				break false;
			}
		} {
			score.win();
		} else {
			score.loose();
		}

		println!("{}", score);
	}
}

fn ask_user_input(word_guess: &str, guess: &mut String) -> Result<()> {
	guess.clear();
	println!("\n{}", word_guess);
	print!("Your guess: ");
	io::stdout().flush().with_context(|| "Can't flush")?;

	io::stdin()
		.read_line(guess)
		.with_context(|| "Can't read line")?;

	Ok(())
}

fn analyse_user_input(guess: &mut str, word: &str, word_guess: &mut String, tries: &mut usize) {
	if let Some(c) = guess.to_lowercase().chars().take(1).last() {
		if c.is_ascii_alphabetic() {
			let matches: Vec<_> = word.match_indices(c).map(|(i, _)| i).collect();
			if matches.is_empty() {
				*tries -= 1;
				println!("Wrong ! {} remaining mistakes", tries);
			} else {
				for i in matches {
					word_guess.replace_range(
						word_guess
							.char_indices()
							.nth(i)
							.map(|(pos, ch)| (pos..pos + ch.len_utf8()))
							.unwrap(),
						&c.to_string(),
					);
				}
			}
		} else {
			println!("Not an alphabetic char, try again");
		}
	} else {
		println!("Not a char");
	}
}

#[test]
fn test_analyse_user_input() {
	let word = "verylongword".to_string();
	let mut word_guess = "____________".to_string();
	let mut tries = 2;

	let mut guess = "l".to_string();
	analyse_user_input(&mut guess, &word, &mut word_guess, &mut tries);
	assert_eq!(word_guess, "____l_______");
	assert_eq!(tries, 2);

	guess = "v".to_string();
	analyse_user_input(&mut guess, &word, &mut word_guess, &mut tries);
	assert_eq!(word_guess, "v___l_______");
	assert_eq!(tries, 2);

	guess = "z".to_string();
	analyse_user_input(&mut guess, &word, &mut word_guess, &mut tries);
	assert_eq!(word_guess, "v___l_______");
	assert_eq!(tries, 1);
}

fn game_is_won(word: &str, word_guess: &str) -> bool {
	let word_guess = word_guess.lines().next().unwrap(); // trim trailing newline, OS agnostic
	if word == word_guess {
		println!("\nYou win ! Complete word is: {}", word.to_uppercase());
		return true;
	}
	false
}

#[test]
fn test_game_is_won() {
	assert_eq!(game_is_won(&"win".to_string(), &"win".to_string()), true);
	assert_eq!(game_is_won(&"win".to_string(), &"loose".to_string()), false);
}

fn game_is_lost(tries: &usize) -> bool {
	if *tries == 0 {
		println!("\nYou lose !");
		return true;
	}
	false
}

#[test]
fn test_game_is_lost() {
	assert_eq!(game_is_lost(&0), true);
	assert_eq!(game_is_lost(&7), false);
}

fn get_lib<T: AsRef<Path> + Display>(filename: T) -> Result<Vec<String>> {
	let file =
		File::open(&filename).with_context(|| format!("Can't open file \"{}\"", filename))?;
	BufReader::new(file)
		.lines()
		.map(|l| l.with_context(|| "Can't parse line into word"))
		.collect()
}

fn get_word(library: &[String]) -> &String {
	&library[rand::thread_rng().gen_range(0..library.len())]
}
