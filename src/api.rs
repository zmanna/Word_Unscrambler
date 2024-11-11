use rand::seq::SliceRandom; // Import SliceRandom to shuffle slices
use reqwest::Client;
use tokio::runtime::Runtime;
use tokio;


pub struct WordApi {
    pub buffer: Vec<String>,
    pub word_length: usize,
    pub num_words: usize,
    pub client: Client
}

pub trait MakeRequest {
    async fn fill_word_buffer(&mut self);
    async fn is_valid_word(&self, word: &str) -> bool;
    fn scramble_word(&self, word: &str) -> String;
    fn get_next_word(&mut self) -> (String, String);
}

impl Default for WordApi {
    fn default() -> Self {
        Self {
            buffer: Vec::new(),
            word_length: 4,
            num_words: 4,
            client: Client::new()
        }
    }
}

/*
if let Some(word) - words.first() {
    let mut chars: Vec<char> word.chars().collect();
    chars.shuffle(&mut rand::thread_rng());
    let scrambled_word: String = chars.into_iter().collect();
    Some((scrambled_word, word.clone()))
} else {
    None
}
    */

impl MakeRequest for WordApi {
    async fn fill_word_buffer(&mut self){
        let url = format!(
            "https://random-word-api.herokuapp.com/word?number={}&&length={}", // URL for random word API fitting length requirements
            self.num_words,
            self.word_length
        );
        let result = 
            self.client
                .get(url)
                .send()
                .await.expect("Request for words failed")
                .text()
                .await.expect("Failed to convert to text");

        self.buffer = result.split(',').map(| word | String::from(word).chars().filter( | c | !['[', ']', '\"'].contains(c)).collect()).collect();
    }

    // Function to check for validity of word referencing the dictionary API
    async fn is_valid_word(&self, word: &str) -> bool {
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);  // URL of dictionary API
        let response = self.client.get(url).send().await;                                          // GET request to API; get response

        // Check if response is successful and return true/false depending on that success
        match response {
            Ok(resp) => resp.status() == 200,
            Err(_) => false,
        }
    }

    fn scramble_word(&self, word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        let scrambled_word: String = chars.into_iter().collect();
        scrambled_word
    }

    fn get_next_word(&mut self) -> (String, String) {
        let buffer_clone = self.buffer.clone();
        let word_lenth_clone = self.word_length.clone();
        let num_words_clone = self.num_words.clone();
        let client_clone = self.client.clone();
        
        if self.buffer.len() == 0 {
            tokio::spawn(async move {
                let mut api_clone = WordApi {
                    buffer: buffer_clone,
                    word_length: word_lenth_clone,
                    num_words: num_words_clone,
                    client: client_clone
                };
                api_clone.fill_word_buffer().await;
            });
            
            return ("loading".into(), "loading".into())
        }
        let original_word = &self.buffer.pop().unwrap();
        let scrambled_word = self.scramble_word(&original_word);
        (scrambled_word, original_word.into())
    }
}

pub fn scramble_word(word: String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.shuffle(&mut rand::thread_rng());
    let scrambled_word: String = chars.into_iter().collect();
    scrambled_word
}