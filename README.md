# wordle-rust

A command-line Wordle game implementation in Rust, featuring a SQLite database for game state persistence and WebAssembly (Wasm) support for core logic.

## Features

*   **Persistent Storage**: Uses SQLite (`wordle.db`) to save game progress.
*   **CLI Interface**: Easy-to-use command-line interface built with `clap`.
*   **Visual Feedback**: Colored output (Green/Yellow/Gray) for guesses in the terminal using ANSI escape codes.
*   **Base64 Encoding**: Answers are stored encoded in the database to prevent accidental spoilers.
*   **Wasm Compatible**: Core logic is separated and can be compiled to WebAssembly.

## Prerequisites

*   Rust (Cargo)

## Installation & Build

1.  Clone the repository.
2.  Build the project:

```bash
cargo build
```

## Usage

Run the game using `cargo run`.

### 1. Start a New Game

Generates a random 5-letter word from the embedded dictionary and creates a new game session.

```bash
cargo run -- new
```
Output: `ID: <GAME_ID>`

### 2. Submit a Guess

Submit a 5-letter word for a specific game ID. The game checks if the word is valid and updates the board.

```bash
cargo run -- submit --id <GAME_ID> --word <GUESS>
# Example
cargo run -- submit --id 1 --word apple
```

### 3. Show Game Status

View the current board, guesses, and status of a game. This displays the colored grid.

```bash
cargo run -- show --id <GAME_ID>
# Example
cargo run -- show --id 1
```

### 4. List All Games

List all created games with their ID, status, and creation time.

```bash
cargo run -- list
```

### 5. Clean Data

Delete all game records and reset the database.

```bash
cargo run -- clean
```

## Project Structure

*   `src/main.rs`: Entry point for the CLI application.
*   `src/cli.rs`: CLI argument definition.
*   `src/game.rs`: Main game logic controller.
*   `src/database.rs`: SQLite database operations (using `rusqlite`).
*   `src/graph.rs`: Terminal visualization logic.
*   `src/misc.rs`: Helper functions (Base64 encryption).
*   `src/lib.rs`: Library entry point for Wasm compilation.
*   `words/words.txt`: Dictionary file embedded into the binary.