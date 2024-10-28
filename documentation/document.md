# Documentation

## Overview

This documentation file goes over everything in the Word Unscrambling Github repo.

---

## Code Artifacts

### 1. `api.rs`

**Description:** Handles API calls to fetch random words and validate if a given word exists in the dictionary.

**Teams's Name:** 23

**Programmer's Name:** Mann

#### Function: `get_scrambled_word(length: usize) -> Option<(String, String)>`

- **Brief Description:** Asynchronously fetches a random word of specified length, scrambles it, and returns both the original and scrambled word.
- **Preconditions:**
  - `length` must be a positive integer representing the desired word length.
- **Postconditions:**
  - Returns `Some((original_word, scrambled_word))` if successful.
  - Returns `None` if the API call fails or no word is retrieved.
- **Error and Exception Conditions:**
  - Network errors during the API call.
  - JSON parsing errors if the API response is malformed.
- **Side Effects:**
  - None.
- **Invariants:**
  - The scrambled word is a permutation of the original word.
- **Known Faults:**
  - The API may return words with unexpected characters.

#### Function: `is_valid_word(word: &str) -> bool`

- **Brief Description:** Asynchronously checks if a given word is valid according to the dictionary API.
- **Preconditions:**
  - `word` must be a non-empty string.
- **Postconditions:**
  - Returns `true` if the word exists in the dictionary.
  - Returns `false` if the word does not exist or an error occurs.
- **Error and Exception Conditions:**
  - Network errors during the API call.
- **Side Effects:**
  - None.
- **Invariants:**
  - None.
- **Known Faults:**
  - The API may have rate limits affecting multiple rapid calls.

---

### 2. `game_state.rs`

**Description:** Defines the `GameState` struct to represent the current state of the game and includes methods to manipulate it.

**Programmer's Name:** Paul, Will

#### Struct: `GameState`

- **Fields:**
  - `score: u32` — Player's current score.
  - `time_left: Duration` — Remaining time for the game.
  - `word_length: usize` — Length of the words to unscramble.
  - `level: u8` — Current level of the game.
  - `original_word: String` — The word to be unscrambled.
  - `scrambled_word: String` — The scrambled version of the original word.

#### Method: `new() -> Self`

- **Brief Description:** Initializes a new game with default settings.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Returns a `GameState` instance with default values.
- **Side Effects:**
  - None.
- **Invariants:**
  - `score` is 0.
  - `time_left` is 60 seconds.
- **Known Faults:**
  - None.

#### Method: `get_new_scrambled_word(&mut self)`

- **Brief Description:** Asynchronously fetches a new word and updates the game state with the original and scrambled versions.
- **Preconditions:**
  - Internet connection is available.
- **Postconditions:**
  - Updates `original_word` and `scrambled_word`.
- **Error and Exception Conditions:**
  - Fails silently if the API call fails.
- **Side Effects:**
  - Modifies the game state.
- **Invariants:**
  - The scrambled word is a valid permutation of the original word.
- **Known Faults:**
  - May not handle API errors gracefully.

#### Method: `increment_word_length(&mut self)`

- **Brief Description:** Increases the word length and level based on the current level.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Increments `word_length` every 7 levels.
  - Increments `level` by 1.
- **Side Effects:**
  - Modifies `word_length` and `level`.
- **Invariants:**
  - `level` increases by 1 each time.
- **Known Faults:**
  - None.

#### Method: `correct_answer(&mut self)`

- **Brief Description:** Updates the game state when the player provides a correct answer.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Increases `score` by 10.
  - Adds 1 second to `time_left`.
- **Side Effects:**
  - Modifies `score` and `time_left`.
- **Invariants:**
  - `score` is non-negative.
- **Known Faults:**
  - None.

#### Method: `incorrect_answer(&mut self)`

- **Brief Description:** Updates the game state when the player provides an incorrect answer.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Decreases `score` by 5, not going below 0.
  - Subtracts 1 second from `time_left`.
- **Side Effects:**
  - Modifies `score` and `time_left`.
- **Invariants:**
  - `score` remains non-negative.
- **Known Faults:**
  - None.

#### Method: `adjust_time(&mut self, seconds: i64)`

- **Brief Description:** Adjusts the remaining time by a specified number of seconds.
- **Preconditions:**
  - `seconds` can be positive or negative.
- **Postconditions:**
  - Updates `time_left`, ensuring it does not become negative.
- **Side Effects:**
  - Modifies `time_left`.
- **Invariants:**
  - `time_left` is always non-negative.
- **Known Faults:**
  - None.

---

## Code Artifacts

### 1. `main.rs`

**Description:** Entry point of the application, integrating the game logic, user interface, and handling user interactions using the `eframe` crate for GUI.

**Programmers:**
- Aryamann Zutshi
- Willem Battey
- Spencer Addis
- John Mosley
- Paul Dykes

**Creation Date:** 10/25/2024

**Revisions:**
- **10/27/2024:**
  - Built the GUI using `eframe` (Paul Dykes).
  - Refactored code to fit the new GUI (John Mosley, Spencer Addis, Aryamann Zutshi, Willem Battey).

#### Module Imports

- `mod game_state;` — Imports the `game_state` module.
- `mod api;` — Imports the `api` module.
- `use eframe::{App, Frame};` — Imports necessary traits for the GUI application.
- `use eframe::egui::{CentralPanel, Context, Key};` — Imports GUI components.
- `use std::sync::mpsc::Receiver;` — For inter-thread communication.
- `use std::time::{Duration, Instant};` — For time tracking.
- `use serde::{Deserialize, Serialize};` — For serialization.

---

#### Struct: `WordUnscramblerApp`

- **Description:** Represents the state of the application, including the game state, user input, timers, and communication channels.
- **Fields:**
  - `game_state: GameState` — The current game state.
  - `input_text: String` — The current text input from the user.
  - `timer_start: Instant` — The start time of the game timer.
  - `validation_receiver: Option<Receiver<(String, bool)>>` — Receiver for input validation results.
  - `scrambled_word_receiver: Option<Receiver<(String, String)>>` — Receiver for scrambled words from the API.
  - `game_over: bool` — Indicates if the game is over.
  - `correct: String` — Message indicating if the last answer was correct or incorrect.

**Preconditions:**
- The application requires an active internet connection to fetch words from the API.

**Postconditions:**
- Maintains the game state throughout the application lifecycle.
- Updates the GUI based on user interactions and game progression.

**Side Effects:**
- Alters the GUI to display new words and game information.

**Invariants:**
- The game runs until the timer ends or the user exits.

**Known Faults:**
- The save feature is not yet integrated into the game.

---

#### Implementation: `Default` for `WordUnscramblerApp`

- **Brief Description:** Initializes `WordUnscramblerApp` with default values.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Sets up a new game with initial settings.
- **Side Effects:**
  - None.
- **Invariants:**
  - `game_over` is `false` upon initialization.
- **Known Faults:**
  - None.

---

#### Trait Implementation: `App` for `WordUnscramblerApp`

- **Function:** `update(&mut self, ctx: &Context, _frame: &mut Frame)`
- **Brief Description:** Updates the application state and GUI every frame.
- **Preconditions:**
  - `self` must be a valid `WordUnscramblerApp` instance.
- **Postconditions:**
  - Renders the GUI, handles user input, updates timers, and processes game logic.
- **Error and Exception Conditions:**
  - None handled explicitly.
- **Side Effects:**
  - Modifies `self` and updates the GUI.
- **Invariants:**
  - The game loop continues until `game_over` is `true`.
- **Known Faults:**
  - None.

**Detailed Behavior:**

- **Game Over Check:**
  - If `game_over` is `true`, displays "Game Over!" and the final score, then returns.
- **Time Update:**
  - Updates `time_left` by subtracting the elapsed time from the total duration.
  - If `time_left` is zero or negative, sets `game_over` to `true`.
- **Validation Receiver Handling:**
  - Checks if a validation result is received.
  - If the answer is correct, updates the game state accordingly.
  - If incorrect, penalizes the player and displays the correct word.
- **Scrambled Word Receiver Handling:**
  - Receives new scrambled words from the API.
  - If no scrambled word is available, requests one from the API.
- **GUI Rendering:**
  - Displays the time left, score, scrambled word, and input field.
  - Handles user input submission.
  - Displays messages indicating whether the last answer was correct or incorrect.
- **Repaint Request:**
  - Requests the GUI to repaint after a short duration.

---

#### Method: `submit_input(&mut self)`

- **Brief Description:** Processes the user's input when they submit a guess.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Validates the user's input and updates the game state.
- **Error and Exception Conditions:**
  - None handled explicitly.
- **Side Effects:**
  - Clears the input field.
  - Spawns a thread to validate the input.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

**Detailed Behavior:**

1. Trims the user's input and checks if it's empty.
2. Clears the `input_text`.
3. Clones the `original_word` from the game state.
4. Creates a channel (`sender`, `receiver`) for validation results.
5. Spawns a background thread to validate the input:
   - Checks if the input is an exact match with the `original_word`.
   - If not, checks if the input is a valid word and an anagram of the `original_word`.
   - Sends the result back through the channel.
6. Sets `validation_receiver` to the created `receiver`.

---

#### Function: `can_form_anagram(input: &str, original: &str) -> bool`

- **Brief Description:** Determines if the `input` is an anagram of `original`.
- **Preconditions:**
  - `input` and `original` are non-empty strings.
- **Postconditions:**
  - Returns `true` if `input` can be formed by rearranging the letters of `original`.
  - Returns `false` otherwise.
- **Error and Exception Conditions:**
  - None.
- **Side Effects:**
  - None.
- **Invariants:**
  - The comparison is case-sensitive.
- **Known Faults:**
  - Does not handle case insensitivity or special characters.

**Implementation Details:**

- Converts both `input` and `original` into character vectors.
- Sorts both vectors.
- Compares the sorted vectors for equality.

---

#### Function: `main()`

- **Brief Description:** Sets up and runs the application using `eframe`.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Launches the GUI application.
- **Error and Exception Conditions:**
  - None handled explicitly.
- **Side Effects:**
  - Initializes the GUI window.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

**Implementation Details:**

- Creates default native options for `eframe`.
- Runs the `WordUnscramblerApp` using `eframe::run_native`.

---

### 2. `api.rs`

*(No changes from the previous documentation, but included here for completeness.)*

**Description:** Handles API calls to fetch random words and validate if a given word exists in the dictionary.

**Programmers:**
- [Your Name]

**Date Created:** [Creation Date]

**Revisions:**
- [Revision Date]: [Description of revision] by [Author's Name]

#### Function: `get_scrambled_word(length: usize) -> Option<(String, String)>`

- **Brief Description:** Fetches a random word of specified length, scrambles it, and returns both the original and scrambled word.
- **Preconditions:**
  - `length` must be a positive integer.
- **Postconditions:**
  - Returns `Some((original_word, scrambled_word))` if successful.
  - Returns `None` if the API call fails.
- **Error and Exception Conditions:**
  - Network errors during the API call.
  - JSON parsing errors.
- **Side Effects:**
  - None.
- **Invariants:**
  - The scrambled word is a permutation of the original word.
- **Known Faults:**
  - The API may return words with unexpected characters.

#### Function: `is_valid_word(word: &str) -> bool`

- **Brief Description:** Checks if a given word is valid according to the dictionary API.
- **Preconditions:**
  - `word` must be a non-empty string.
- **Postconditions:**
  - Returns `true` if the word exists in the dictionary.
  - Returns `false` otherwise.
- **Error and Exception Conditions:**
  - Network errors during the API call.
- **Side Effects:**
  - None.
- **Invariants:**
  - None.
- **Known Faults:**
  - The API may have rate limits affecting multiple rapid calls.

---

### 4. `save_load.rs`

**Description:** Provides functionality to save the current game state to a file and load it back.

**Programmer's Name:** Mann, John

#### Function: `save_game(game_state: &GameState)`

- **Brief Description:** Saves the game state to a JSON file named `save_game.json`.
- **Preconditions:**
  - `game_state` is a valid instance of `GameState`.
- **Postconditions:**
  - Serializes `game_state` and writes it to a file.
- **Error and Exception Conditions:**
  - File write errors if the file system is inaccessible.
- **Side Effects:**
  - Creates or overwrites `save_game.json`.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

#### Function: `load_game() -> Result<GameState, &'static str>`

- **Brief Description:** Loads the game state from `save_game.json` if it exists.
- **Preconditions:**
  - `save_game.json` exists and contains valid JSON.
- **Postconditions:**
  - Returns `Ok(GameState)` if successful.
  - Returns `Err` with an error message if loading fails.
- **Error and Exception Conditions:**
  - File read errors.
  - JSON deserialization errors.
- **Side Effects:**
  - Reads from the file system.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

---

#### Function: `get_user_input() -> String`

- **Brief Description:** Reads input from the user and returns it as a trimmed string.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Returns the user's input without leading/trailing whitespace.
- **Error and Exception Conditions:**
  - Input errors if the standard input is not available.
- **Side Effects:**
  - Reads from the console.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

---

## Comments and Annotations

- **Major Blocks:** Each module and function includes comments explaining its purpose and behavior.
- **Line-by-Line Comments:** Complex logic within functions is commented to enhance understanding.
- **4GLs Comments:** Not applicable; the code is written in Rust, a general-purpose programming language.

---

## Testing and Quality Assurance

- **Unit Tests:** Not included in the code but recommended for each function, especially API calls and game logic.
- **Known Issues:**
  - Network connectivity is required; the game does not handle offline scenarios.
  - Error handling is minimal for simplicity; robust error handling is advised.
  - Saving and loading may fail silently; adding user feedback is recommended.
- **Future Improvements:**
  - Implement retry logic for API calls.
  - Enhance error messages and exception handling.
  - Add unit tests to cover critical functions.
