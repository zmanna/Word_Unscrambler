use rand::seq::SliceRandom; // Import SliceRandom to shuffle slices
use reqwest::Client;
use reqwest::Response;
use tokio::runtime::Runtime;
use tokio::{self, sync::Notify};
use std::sync::{Arc, Mutex};


pub struct WordApi {
    pub buffer: Arc<Mutex<Vec<String>>>,
    pub word_length: usize,
    pub num_words: usize,
    pub client: Client,
    pub notify: Arc<Notify>,
    pub requested: bool,
}

pub trait MakeRequest {
    fn is_valid_word(&self, word: &str) -> bool;
    fn scramble_word(&self, word: &str) -> String;
}

impl Default for WordApi {
    fn default() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            word_length: 4,
            num_words: 4,
            client: Client::new(),
            notify: Arc::new(Notify::new()),
            requested: false,
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
    // Function to check for validity of word referencing the dictionary API
    fn is_valid_word(&self, word: &str) -> bool {
        let valid = Arc::new(Mutex::new(None));
        let valid_arc: Arc<Mutex<Option<bool>>> = Arc::clone(&valid);
        let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
        tokio::spawn(async move{
            let response = reqwest::get(&url).await;

            match response{
                Ok(resp) => *valid_arc.lock().unwrap() = Some(resp.status() == 200),
                Err(_) => *valid_arc.lock().unwrap() =  Some(false)}          
        });
        while *valid.lock().unwrap() == None{};
        let result = valid.lock().unwrap();
        match *result{
            Some(res) => res,
            None => {eprint!{"Error returning validation response..."}; false},
        }
}



    fn scramble_word(&self, word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        let scrambled_word: String = chars.into_iter().collect();
        scrambled_word
    }
}

impl WordApi {
    pub fn get_next_word(&mut self) -> Option<(String, String)> {
        {
            let mut buffer = self.buffer.lock().unwrap();

            if let Some(original_word) = buffer.pop() {
                if buffer.is_empty(){ self.word_length += 1 }; //Increment word length if last word in buffer
                let scrambled_word = self.scramble_word(&original_word);
                println!("Words Remaining: {}", buffer.len());
                return Some((scrambled_word, original_word));
            }
        } // Release the lock before potentially spawning async task

        // Buffer is empty, spawn task to fill it
        if !self.requested{
        let buf = Arc::clone(&self.buffer);
        let word_length = self.word_length.clone();
        let num_words = self.num_words.clone();
        let client = self.client.clone();

        let notify = self.notify.clone();
        tokio::spawn(async move {
            fill_word_buffer(num_words, word_length, client, buf).await;
            notify.notify_one();
        });
        }

        None // Return None since we have no word to provide yet
    }
}

async fn fill_word_buffer(num_words: usize, word_length: usize, client: Client, buf: Arc<Mutex<Vec<String>>>) -> Arc<Mutex<Vec<String>>>{
    let url = format!(
        "https://random-word-api.herokuapp.com/word?number={}&&length={}", // URL for random word API fitting length requirements
        num_words,
        word_length
    );
    let result = client
        .get(url)
        .send()
        .await
        .expect("Request for words failed")
        .text()
        .await
        .expect("Failed to convert to text");

    let words = result
        .split(',')
        .map(| word | String::from(word).chars().filter( | c | !['[', ']', '\"'].contains(c)).collect())
        .collect::<Vec<String>>();

    for word in words{ buf.lock().unwrap().push(word) }
    
    return buf
}

pub fn scramble_word(word: String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.shuffle(&mut rand::thread_rng());
    let scrambled_word: String = chars.into_iter().collect();
    scrambled_word
}

