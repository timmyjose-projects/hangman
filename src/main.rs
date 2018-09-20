extern crate hangman;
extern crate rand;

use std::str::FromStr;
use rand::{thread_rng, Rng};


fn main() {
    let mut max_word_length = hangman::MAX_WORD_LENGTH;
    let mut number_of_attempts = hangman::MIN_NUMBER_OF_ATTEMPTS;

    // check the params
    let args = std::env::args().skip(1)
        .map(|w| i32::from_str(&w).unwrap())
        .collect::<Vec<i32>>();

    if args.len() == 0 || args.len() > 2 {
        hangman::display_usage();
    } else if args.len() == 1 {
        hangman::check_and_set_max_word_length(args[0], &mut max_word_length);
    } else {
        hangman::check_and_set_max_word_length(args[0], &mut max_word_length);
        hangman::check_and_set_attempts(args[1], &mut number_of_attempts);
    }
    
    // load the words file
    let words = hangman::load_words_file(max_word_length);

    // start the game
    play_game(words, number_of_attempts);
}


// the main game loop
fn play_game(words: Vec<String>, max_attempts: i32) {
    loop {
        println!("\nWelcome to hangman! You have {} attempts!", max_attempts);
        
        let mut rng = thread_rng();
        let rand_idx = rng.gen_range(0, words.len());
        let random_word = &words[rand_idx].to_lowercase();

        let mut attempts = 0;
        let mut word = hangman::init_word(random_word.len());
        
        while attempts <= max_attempts {
            hangman::display_word(&word);
            hangman::update_word(&random_word, &mut word, &mut attempts);

            if hangman::check_game_solved(&random_word, &word) {
                println!("\nYou win! You took {} attempts to crack the word \"{}\"!", attempts, random_word);
                break;
            }

            if attempts > max_attempts {
                println!("\nSorry, but you exceeded the maximum number of attempts to guess \"{}\". Better luck next time!", random_word);
                break;
            }
        }

        // check if the user wants to continue the game
        if let Some(_) =  hangman::check_if_continue() {
            continue;
        } else {
            break;
        }
    }

    println!("\nThank you for playing Hangman!\n");
}
