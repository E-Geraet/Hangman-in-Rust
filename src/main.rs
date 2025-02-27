use std::io::{self, Write};
use rand::seq::SliceRandom;
use std::process::Command;

// Define the Game struct to hold game state
struct Game {
    word: String,               // The word to be guessed
    attempts_left: u8,           // Number of incorrect guesses allowed
    guessed_letters: Vec<char>, // List of letters already guessed
}

impl Game {
    // Create a new game with a random word from the word bank
    fn new(word_bank: &[&str]) -> Self {
        // Select a random word from the provided word bank
        let word = word_bank.choose(&mut rand::thread_rng()).unwrap().to_string();
        Game {
            word,
            attempts_left: 7, // Set the initial number of attempts
            guessed_letters: Vec::new(), // Initialize with no guessed letters
        }
    }

    // Display the current word with guessed letters and underscores
    fn display_word(&self) -> String {
        let mut display_word = String::new();
        for c in self.word.chars() {
            // If the letter has been guessed, show it, otherwise show an underscore
            if self.guessed_letters.contains(&c) {
                display_word.push(c);
            } else {
                display_word.push('_');
            }
        }
        display_word // Return the string representing the current word
    }

    // Display the hangman figure based on the number of attempts left
    fn display_hangman(&self) {
        let image_path = format!("images/stage_{}.png", 7 - self.attempts_left);
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "start", &image_path])
                .spawn()
                .expect("Failed to open image");
        } else if cfg!(target_os = "macos") {
            Command::new("open")
                .arg(image_path)
                .spawn()
                .expect("Failed to open image");
        } else if cfg!(target_os = "linux") {
            Command::new("xdg-open")
                .arg(image_path)
                .spawn()
                .expect("Failed to open image");
        }
    }

    // Handle a letter guess
    fn guess(&mut self, letter: char) {
        if !self.guessed_letters.contains(&letter) { // Check if the letter hasn't been guessed already
            if self.word.contains(letter) { // If the letter is in the word
                self.guessed_letters.push(letter); // Add it to guessed letters
                println!("Correct guess!");
            } else { // If the letter is not in the word
                self.attempts_left -= 1; // Decrease attempts left
                println!("Incorrect guess.");
            }
        } else {
            println!("You already guessed that letter!"); // Inform the user if they've guessed the letter already
        }
    }

    // Check if the word has been fully guessed (all letters guessed)
    fn is_fully_guessed(&self) -> bool {
        // Check if all characters in the word have been guessed
        self.word.chars().all(|c| self.guessed_letters.contains(&c))
    }
}

fn main() {
    // Define a list of possible words for the game
    let word_bank = ["rustacean", "programming", "hangman", "rust", "challenge"];
    let mut game = Game::new(&word_bank); // Initialize a new game with a random word

    println!("Welcome to Hangman!");

    // Main game loop
    loop {
        // Display the current hangman figure and word
        game.display_hangman();
        println!("\nCurrent word: {}", game.display_word());
        println!("Attempts left: {}", game.attempts_left);

        // Get user input for the next guess
        print!("Enter a letter: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is printed

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap(); // Read the user input
        let guess = guess.trim().to_lowercase(); // Trim whitespace and convert to lowercase

        // Only allow a single letter guess
        if guess.len() == 1 {
            let letter = guess.chars().next().unwrap(); // Get the first character of the input
            game.guess(letter); // Handle the guess
        } else {
            println!("Please enter only one letter.");
        }

        // Check if the game is over due to running out of attempts
        if game.attempts_left == 0 {
            println!("\nGame Over! The word was: {}", game.word);
            break; // Exit the game loop
        }

        // Check if the word is fully guessed
        if game.is_fully_guessed() {
            println!("\nCongratulations! You guessed the word: {}", game.word);
            break; // Exit the game loop when the word is fully guessed
        }
    }
}
