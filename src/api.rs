use rand::seq::SliceRandom;
use ureq;

pub fn get_scrambled_word(length: usize) -> Option<(String, String)> {
    let url = format!(
        "https://random-word-api.herokuapp.com/word?number=1&length={}",
        length
    );
    let response = ureq::get(&url).call().ok()?;
    let words: Vec<String> = response.into_json().ok()?;

    if let Some(word) = words.first() {
        let mut chars: Vec<char> = word.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        let scrambled_word: String = chars.into_iter().collect();
        Some((scrambled_word, word.clone()))
    } else {
        None
    }
}

pub fn is_valid_word(word: &str) -> bool {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    let response = ureq::get(&url).call();

    match response {
        Ok(resp) => resp.status() == 200,
        Err(_) => false,
    }
}
