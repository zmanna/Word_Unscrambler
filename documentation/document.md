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

### 3. `main.rs`

**Description:** Entry point of the application, orchestrating the game loop, handling user interactions, and integrating all modules.

**Programmer's Name:** Mann, Spencer, Will
#### Function: `main()`

- **Brief Description:** Initializes the game, enters the game loop, and handles game over conditions.
- **Preconditions:**
  - None.
- **Postconditions:**
  - Runs the game until time runs out or the player quits.
- **Side Effects:**
  - Reads user input and writes to the console.
- **Invariants:**
  - None.
- **Known Faults:**
  - None.

#### Function: `can_form_anagram(input: &str, original: &str) -> bool`

- **Brief Description:** Checks if the input word is an anagram of the original word.
- **Preconditions:**
  - `input` and `original` are non-empty strings.
- **Postconditions:**
  - Returns `true` if `input` is an anagram of `original`.
  - Returns `false` otherwise.
- **Side Effects:**
  - None.
- **Invariants:**
  - The sorted characters of both words are compared.
- **Known Faults:**
  - None.

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

### 5. `ui.rs`

**Description:** Manages user interface elements, including displaying game information and capturing user input.

**Programmer's Name:** Paul
#### Function: `display_scrambled(game_state: &GameState)`

- **Brief Description:** Clears the console and displays the current game state, including time left, score, and the scrambled word.
- **Preconditions:**
  - `game_state` is a valid `GameState` instance.
- **Postconditions:**
  - Updates the console display with current game information.
- **Error and Exception Conditions:**
  - Terminal commands may fail on unsupported consoles.
- **Side Effects:**
  - Clears and writes to the console.
- **Invariants:**
  - None.
- **Known Faults:**
  - May not work properly on non-standard terminals.

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
