use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
use std::time::Instant;
use termion::{color, style};

// Define difficulty levels
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

// Define game state
struct GameState {
    secret_number: u32,
    attempts: u32,
    max_attempts: u32,
    score: u32,
    start_time: Instant,
    difficulty: Difficulty,
}

impl GameState {
    fn new(difficulty: Difficulty) -> Self {
        let (range, max_attempts) = match difficulty {
            Difficulty::Easy => (1..=50, 10),
            Difficulty::Medium => (1..=100, 7),
            Difficulty::Hard => (1..=200, 5),
        };

        GameState {
            secret_number: rand::thread_rng().gen_range(range),
            attempts: 0,
            max_attempts,
            score: 1000,
            start_time: Instant::now(),
            difficulty,
        }
    }

    fn make_guess(&mut self, guess: u32) -> Ordering {
        self.attempts += 1;
        self.score = self.score.saturating_sub(50); // Deduct points for each attempt
        guess.cmp(&self.secret_number)
    }

    fn is_game_over(&self) -> bool {
        self.attempts >= self.max_attempts
    }

    fn calculate_final_score(&self) -> u32 {
        let time_bonus = match self.difficulty {
            Difficulty::Easy => 5,
            Difficulty::Medium => 10,
            Difficulty::Hard => 20,
        };
        let time_taken = self.start_time.elapsed().as_secs();
        self.score
            .saturating_add(time_bonus * (60u32.saturating_sub(time_taken as u32)))
    }
}

fn get_difficulty() -> Difficulty {
    loop {
        println!("Choose difficulty:");
        println!("1. Easy (1-50, 10 attempts)");
        println!("2. Medium (1-100, 7 attempts)");
        println!("3. Hard (1-200, 5 attempts)");
        print!("Enter your choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => return Difficulty::Easy,
            "2" => return Difficulty::Medium,
            "3" => return Difficulty::Hard,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn play_game(mut game: GameState) -> bool {
    println!("{}{}Guess the number!", color::Fg(color::Blue), style::Bold);
    println!(
        "You have {} attempts. Good luck!{}",
        game.max_attempts,
        style::Reset
    );

    while !game.is_game_over() {
        print!(
            "{}Attempt {}/{}: ",
            color::Fg(color::Green),
            game.attempts + 1,
            game.max_attempts
        );
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{}Please enter a valid number!{}",
                    color::Fg(color::Red),
                    style::Reset
                );
                continue;
            }
        };

        match game.make_guess(guess) {
            Ordering::Less => println!("{}Too small!{}", color::Fg(color::Yellow), style::Reset),
            Ordering::Greater => println!("{}Too big!{}", color::Fg(color::Yellow), style::Reset),
            Ordering::Equal => {
                println!(
                    "{}{}You win!{}",
                    color::Fg(color::Green),
                    style::Bold,
                    style::Reset
                );
                println!("Final score: {}", game.calculate_final_score());
                return true;
            }
        }

        println!("Current score: {}", game.score);
    }

    println!(
        "{}{}Game over! You've run out of attempts.{}",
        color::Fg(color::Red),
        style::Bold,
        style::Reset
    );
    println!("The secret number was: {}", game.secret_number);
    false
}

fn main() {
    loop {
        let difficulty = get_difficulty();
        let game = GameState::new(difficulty);
        play_game(game);

        print!("{}Play again? (y/n): ", color::Fg(color::Blue));
        io::stdout().flush().unwrap();
        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");
        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
    println!("Thanks for playing!");
}
