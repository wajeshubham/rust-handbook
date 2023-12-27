# Guessing Game in Rust

This is a simple command-line guessing game written in Rust. The game generates a random number between 1 and 100, and the player tries to guess it.

# Prerequisites

To run this project, you need to have Rust and Cargo (the Rust package manager) installed on your machine. You can download and install them from [the official Rust website](https://www.rust-lang.org/learn/get-started).

# Creating a New Cargo Project

To create a new Cargo project for this guessing game, follow these steps:

1. **Create a new project:**

   Open your terminal and run:

   ```bash
   cargo new guessing_game
   cd guessing_game
   ```

   This command creates a new directory named `guessing_game` with a Cargo project inside it.

2. **Add Dependencies:**

   Open `Cargo.toml` in the project directory and add the following under `[dependencies]`:

   ```toml
   rand = "0.8.5"
   ```

   This adds the `rand` crate as a dependency, which is used for generating random numbers.

3. **Write the Game Code:**

   Open `src/main.rs` and write your game code in Rust.

4. **Build the Project:**

   Build your project using:

   ```bash
   cargo build
   ```

   This command compiles the project and generates an executable in the `target/debug` directory.

# Running the Project

Run your project using:

```bash
cargo run
```

This command compiles the project (if necessary) and then runs the executable.

# How to Play

- The program will prompt you to guess a number between 1 and 100.
- Enter your guess and press Enter.
- The program will tell you if your guess is too high, too low, or correct.
- Keep guessing until you find the right number.
- The game ends when you guess the correct number.
