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
    // println!("random word: {}", secret_word);
    let mut guessed: Vec<char> = vec![];

    // Your code here! :)
    let mut guess_left = 5;
    while guess_left > 0 {
        let mut success = true;
        println!(
            "The word so far is {}",
            secret_word_chars
                .iter()
                .map(|c| if guessed.contains(c) {
                    *c
                } else {
                    success = false;
                    '-'
                })
                .collect::<String>()
        );

        if success {
            println!("Congratulations you guessed the secret word");
            return;
        }

        println!(
            "You have guessed the following letters: {}",
            guessed.iter().collect::<String>()
        );
        println!("You have {} guesses left", guess_left);
        print!("Please guess a letter: ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("failed to read input");
        match buf.trim().chars().nth(0) {
            Some(c) => {
                if guessed.contains(&c) {
                    println!("You have already guessed this character");
                } else {
                    guessed.push(c);
                    if !secret_word_chars.contains(&c) {
                        guess_left -= 1;
                        println!("Sorry, that letter is not in the word");
                    }
                }
            }
            None => {
                println!("please input a character!");
            }
        }
    }
}
