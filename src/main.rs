use rand::Rng;
use std::io::{self, Write};
use std::str;

fn main() {
    let library = get_lib();
    let word = get_word(library);
    let mut word_guess = str::repeat("_", word.len());

    let mut tries: usize = 10usize;
    let mut guess = String::new();

    loop {
        println!("\n{}", word_guess);
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failure to read user input");

        // Enable complete-word guessing
        if game_is_won(word, &guess) {
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
                    if game_is_won(word, &word_guess) {
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

fn game_is_won(word: &str, word_guess: &String) -> bool {
    let word_guess = word_guess.lines().next().unwrap().to_string(); // trim trailing newline, OS agnostic
    if word == word_guess {
        println!("\nYou win ! Complete word is: {}", word.to_uppercase());
        return true;
    }
    false
}

#[test]
fn test_game_is_won() {
    assert_eq!(game_is_won(&"win", &"win".to_string()), true);
    assert_eq!(game_is_won(&"win", &"loose".to_string()), false);
}

fn get_lib<'a>() -> Vec<&'a str> {
    vec![
        "absolument",
        "abominable",
        "annulation",
        "assassiner",
        "attirant",
        "balancier",
        "beneficier",
        "bijouterie",
        "clarinette",
        "decoration",
        "ecologiste",
        "entraineur",
        "facultatif",
        "fracassant",
        "galanterie",
        "guitariste",
        "hippocampe",
        "inegalable",
        "journalier",
        "mecanicien",
        "menuiserie",
        "modulable",
        "ondulation",
        "pirouette",
        "rearranger",
        "recomposer",
        "rectiligne",
        "sagittaire",
        "salutation",
        "sanctuaire",
        "sectoriser",
        "simulateur",
        "strasbourg",
        "torrentiel",
        "utilitaire",
        "vandaliser",
        "verbaliser",
        "vibratoire",
    ]
}

fn get_word(library: Vec<&str>) -> &str {
    library[rand::thread_rng().gen_range(0..library.len())]
}
