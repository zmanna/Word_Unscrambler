use rand::seq::SliceRandom; // Import SliceRandom to shuffle slices
use ureq;                   // Import ureq for HTTP requests

// Function to get random word from API return scrambled version
pub fn get_scrambled_word(length: usize) -> Option<(String, String)> {
    let url = format!(
        "https://random-word-api.herokuapp.com/word?number=1&length={}", // URL for random word API fitting length requirements
        length
    );
    let response = ureq::get(&url).call().ok()?;                         // GET request to API; get response
    let words: Vec<String> = response.into_json().ok()?;                 // parse response into JSON vector of words (none if it fails to parse)

    if let Some(word) = words.first() {                                  // If word has been retrieved
        let mut chars: Vec<char> = word.chars().collect();               // Split original word into vector of chars
        chars.shuffle(&mut rand::thread_rng());                          // Scramble chars 
        let scrambled_word: String = chars.into_iter().collect();        // Form string from scrambled chars
        Some((scrambled_word, word.clone()))                             // Tuple containing orginal and scrambled word
    } else {                                                             // If no word retrieved, return none
        None
    }
}

// Function to check for validity of word referencing the dictionary API
pub fn is_valid_word(word: &str) -> bool {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);  // URL of dictionary API
    let response = ureq::get(&url).call();                                          // GET request to API; get response

    // Check if response is successful and return true/false depending on that success
    match response {
        Ok(resp) => resp.status() == 200,
        Err(_) => false,
    }
}
