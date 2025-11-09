// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    let mut left_guess = NUM_INCORRECT_GUESSES;
    let mut guessed_chars: Vec<char> = secret_word_chars.clone();
    for item in guessed_chars.iter_mut() {
        *item = '-';
    }
    let mut tried_letter = String::new();
    while left_guess > 0 {
        println!("The word so far is {}", guessed_chars.clone().into_iter().collect::<String>());
        println!("You have guessed the following letters: {}", tried_letter);
        println!("You have {} guesses left", left_guess);

        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let mut hit_idx: Option<usize> = None;
        for (idx, var) in secret_word_chars.iter().enumerate() {
            if guessed_chars[idx] == '-' && *var == guess.chars().next().unwrap() {
                hit_idx = Some(idx);
                break
            }
        }
        if hit_idx.is_none() {
            println!("Sorry, that letter is not in the word");
            tried_letter.push_str(guess.as_str().trim());
            left_guess -= 1;
        }
        else {
            guessed_chars[hit_idx.unwrap()] = guess.chars().next().unwrap();
            if guessed_chars.clone().into_iter().collect::<String>() == secret_word {
                println!("Congratulations you guessed the secret word: {}!", secret_word);
                return;
            }
        }
    }
    println!("Sorry, you ran out of guesses!");
}
