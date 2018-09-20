use std::io::{self, BufRead, BufReader, Write};
use std::fs::{self, File};
use std::process::Command;

pub const MIN_WORD_LENGTH: i32 = 5;
pub const MAX_WORD_LENGTH: i32 = 24;

pub const MIN_NUMBER_OF_ATTEMPTS: i32 = 10;
pub const MAX_NUMBER_OF_ATTEMPTS: i32 = 100;


/// general validation

pub fn display_usage() {
    writeln!(io::stderr(), "Usage: hangman [max-word-length = 5 to 24] [number-of-attempts = 10 to 100]").unwrap();
    std::process::exit(1);
}


pub fn check_and_set_max_word_length(val: i32, len: &mut i32) {
    if val < MIN_WORD_LENGTH || val > MAX_WORD_LENGTH {
        writeln!(io::stderr(), "Word length must be between 5 and 24 (inclusive)").unwrap();
        std::process::exit(2);
    }

    *len = val;
}


/// 
pub fn check_and_set_attempts(val: i32, len: &mut i32) {
    if val < MIN_NUMBER_OF_ATTEMPTS || val > MAX_NUMBER_OF_ATTEMPTS {
        writeln!(io::stderr(), "Number of attempts must be between 10 and 100 (inclusive)").unwrap();
        std::process::exit(3);
    }

    *len = val;
}


/// load the words file

/// If the OS is UNIX-like, try loading the /usr/share/dicts/words file,
/// otherwise load the resources/words.txt file
pub fn load_words_file(max_word_len: i32) -> Vec<String> {
    const SYSTEM_WORDS_FILE: &'static str = "/usr/share/dict/words";
    const USER_WORDS_FILE: &'static str = "/resources/words.txt";

    // need this to ensure that running the executable also finds
    // the resources file correctly (if needed)
    let mut user_words_file_path = String::from(env!("CARGO_MANIFEST_DIR"));
    user_words_file_path.push_str(USER_WORDS_FILE);
    
    let mut words = Vec::new();
    
    if cfg!(target_os = "linux") || cfg!(target_os = "darwin") {
        match fs::metadata(SYSTEM_WORDS_FILE) {
            Ok(_) => load_system_file(SYSTEM_WORDS_FILE, &mut words, max_word_len).unwrap(),
            Err(_) => load_user_file(&user_words_file_path, &mut words, max_word_len).unwrap(),
        }
    } else {
        load_user_file(&user_words_file_path, &mut words, max_word_len).unwrap();
    }

    words
}

fn load_user_file(file: &str, words: &mut Vec<String>, max_len: i32) -> io::Result<()> {
    let reader = BufReader::new(File::open(file)?);

    for line in reader.lines() {
        let line = line?;

        if line.len() as i32 >= MIN_WORD_LENGTH && line.len() as i32 <= max_len {
            words.push(line);
        }
    }

    Ok(())
}

fn load_system_file(file: &str, words: &mut Vec<String>, max_len: i32) -> io::Result<()> {
    let cmd = Command::new("cat")
        .arg(file)
        .output()
        .expect("could not open the words file!");

    let data = String::from_utf8(cmd.stdout).unwrap();

    for word in data.lines() {
        if word.len() as i32 >= MIN_WORD_LENGTH && word.len() as i32 <= max_len {
            words.push(String::from(word));
        }
    }

    Ok(())
}


/// game logic

pub fn init_word(len: usize) -> String {
    let mut word = String::with_capacity(len);

    for _ in 0..len {
        word.push('_');
    }

    word
}


pub fn display_word(word: &str) {
    for c in word.chars() {
        print!("{} ", c);
    }
    println!();
}


/// Update the word with all occurrences of the entered character
// (if any match) otherwise return the word unchanged
pub fn update_word(random_word: &str, word: &mut String, attempts: &mut i32) {
    let input_char = get_char("\nEnter your guess: ");

    let mut idx = 0;
    
    for c in random_word.chars() {
        if c == input_char {
            word.remove(idx);
            word.insert(idx, input_char);
        }
        idx += 1;
    }

    *attempts += 1;
}

fn get_char(prompt: &str) -> char {
    print!("{}", prompt);

    io::stdout().flush().unwrap();
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("no char!");

    input.trim()
        .chars()
        .nth(0)
        .unwrap()
}


pub fn check_game_solved(random_word: &str, word: &str) -> bool {
    random_word == word
}


/// Check if the user wants to continue the game, or not
pub fn check_if_continue() -> Option<bool> {
    match get_choice("\nDo you want to continue? [y/n]: ") {
        true => Some(true),
        false => None,
    }
}

fn get_choice(prompt: &str) -> bool {
    print!("{}", prompt);

    io::stdout().flush().unwrap();
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("no input!");

    match input.trim() {
        "y" | "Y" | "Yes" | "yes" | "yeah" | "Yeah" => true,
        _ => false,
    }
}

