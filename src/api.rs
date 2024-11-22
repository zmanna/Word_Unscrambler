use rand::seq::SliceRandom; // Import SliceRandom to shuffle slices
use reqwest::Client;
use reqwest::Response;
use tokio::runtime::Runtime;
use tokio::{self, sync::Notify};
use core::error;
use std::default;
use std::sync::{Arc, Mutex};
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::contact_server::send_recieve::MakeRequest;
use crate::contact_server::send_recieve::User;
use crate::contact_server::send_recieve::ReturnType;


pub struct WordApi {
    pub buffer: Arc<Mutex<Vec<String>>>,
    pub word_length: usize,
    pub num_words: usize,
    pub client: Client,
    pub notify: Arc<Notify>,
    pub requested: bool,
}

impl default::Default for WordApi {
    fn default() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            word_length: 5,
            num_words: 10,
            client: Client::new(),
            notify: Arc::new(Notify::new()),
            requested: false,
        }
    }
}

pub struct DbAPI {
    pub client: Client,
    pub notify: Arc<Notify>,
    pub friends: Arc<Mutex<Vec<User>>>,
    pub users: Arc<Mutex<Vec<User>>>,
}


impl DbAPI {

    pub fn new() -> Self{
        let api = DbAPI {
            client: Client::new(),
            notify: Arc::new(Notify::new()),
            friends: Arc::new(Mutex::new(Vec::new())),
            users: {Arc::new(Mutex::new(Vec::new()))}
        };
        api.send_request("/api/User/GetAllUsers");

        api
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

    fn scramble_word(&self, word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.shuffle(&mut rand::thread_rng());
        let scrambled_word: String = chars.into_iter().collect();
        scrambled_word
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