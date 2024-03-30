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

fn print_info(counter: &u32, guessed_word: &Vec<char>,guessed_letter: &Vec<char>){
    println!("The word so far is {}",guessed_word.clone().into_iter().collect::<String>());
    println!("You have guessed the following letters: {}",guessed_letter.iter().collect::<String>());
    println!("You have {} guesses left",counter);
    print!("Please enter your guess: ");
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    
    println!("Welcome to CS110L Hangman!");
    let mut counter = NUM_INCORRECT_GUESSES;
    let mut guessed_word: Vec<char> = vec!['-'; secret_word.len()];
    let mut guessed_letter: Vec<char> = Vec::new();
    let found_flag = false;

    // for c in &secret_word_chars{
    //     print!("{}",c);
    // }
    // println!();

    while found_flag == false && counter > 0 {
        print_info(&counter,&guessed_word,&guessed_letter);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().chars().next().unwrap();
        
        guessed_letter.push(guess);

        let mut found = false;
        for i in 0..secret_word.len(){
            if secret_word_chars[i] == guess{
                guessed_word[i] = guess;
                found = true;
            }
        }
        if found == false{
            counter -= 1;
            if counter == 0{
                break;
            }
            println!("Incorrect! You have {} guesses left", counter);
        }

        println!();
        if guessed_word.iter().all(|&c| c != '-'){
            println!("Congratulations! You guessed the word: {}", secret_word);
            break;
        }
    }

    if counter == 0{
        println!("\nSorry, you ran out of guesses! The word was: {}", secret_word);
    }

}
