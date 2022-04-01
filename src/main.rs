use rand::Rng;
use std::{
    self,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
    str,
};

fn main() {
    let library = get_lib("library.txt");
    let mut word_guess;
    let mut tries: usize;
    let mut guess = String::new();

    'game: loop {
        guess.clear();
        tries = 10;
        let word = get_word(&library);
        word_guess = str::repeat("_", word.len());

        loop {
            println!("\n{}", word_guess);
            print!("Your guess: ");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failure to read user input");

            if guess.starts_with("quit") {
                break 'game;
            }

            // Enable complete-word guessing
            if game_is_won(&word, &guess) {
                break;
            }

            if let Some(c) = guess.to_lowercase().chars().take(1).last() {
                if c.is_ascii_alphabetic() {
                    let matches: Vec<_> = word.match_indices(c).map(|(i, _)| i).collect();
                    if matches.len() == 0 {
                        tries -= 1;
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
                        if game_is_won(&word, &word_guess) {
                            break;
                        }
                    }
                } else {
                    println!("Not an alphabetic char, try again");
                }
                guess.clear();
            } else {
                println!("Not a char");
            }

            if tries == 0 {
                println!("\nYou lose !");
                break;
            }
        }
    }
}

fn game_is_won(word: &String, word_guess: &String) -> bool {
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

fn get_lib(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_word(library: &Vec<String>) -> &String {
    &library[rand::thread_rng().gen_range(0..library.len())]
}
