use std::fs;
use serde_json;
use crate::game_state::GameState;

pub fn save_game(game_state: &GameState) {
    let serialized = serde_json::to_string(game_state).expect("Failed to serialize game state");
    fs::write("save_game.json", serialized).expect("Failed to save game.");
}

pub fn load_game() -> Result<GameState, &'static str> {
    let data = fs::read_to_string("save_game.json").map_err(|_| "Failed to load game.")?;
    serde_json::from_str(&data).map_err(|_| "Failed to deserialize game state.")
}
