# Guessing Rust

Guessing Rust is an advanced command-line guessing game written in Rust. It features multiple difficulty levels, a scoring system, and colorful terminal output.

## Features

- Three difficulty levels: Easy, Medium, and Hard
- Dynamic scoring system based on attempts and time taken
- Colorful terminal output for enhanced user experience
- Replay option to enjoy multiple game sessions

## Prerequisites

Before you begin, ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/developtheweb/guessing_rust.git
   cd guessing_rust
   ```

2. Build the project:
   ```
   cargo build
   ```

## How to Play

1. Run the game:
   ```
   cargo run
   ```

2. Choose a difficulty level:
   - Easy: Numbers from 1 to 50, 10 attempts
   - Medium: Numbers from 1 to 100, 7 attempts
   - Hard: Numbers from 1 to 200, 5 attempts

3. Start guessing! The game will provide feedback on whether your guess is too high or too low.

4. Try to guess the number within the given attempts to win.

5. Your score is calculated based on the number of attempts and time taken. The faster you guess, the higher your score!

6. After each game, you can choose to play again or exit.

## Project Structure

- `src/main.rs`: Contains the main game logic
- `Cargo.toml`: Defines project dependencies

## Dependencies

This project uses the following external crates:
- `rand`: For generating random numbers
- `termion`: For colorful terminal output

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- Rust Programming Language Team
- Contributors to the `rand` and `termion` crates

Enjoy the game and happy coding!
