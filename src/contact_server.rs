
pub mod send_recieve
{
    use std::sync::{Arc, Mutex};
    use crate::api::{WordApi, DbAPI};
    use serde::Deserialize;
    pub trait MakeRequest {
        fn send_request(&self, input: &str) -> ReturnType;    
    }

    pub enum ReturnType{
        IsValid(bool),
        Users(Vec<User>),
        Error(Option<String>),
    }

    #[derive(Deserialize)]
    pub struct User{
        username: String,
        score: u32,
    }

    impl MakeRequest for WordApi {
        // Function to check for validity of word referencing the dictionary API
        fn send_request(&self, input: &str) -> ReturnType {
            let valid = Arc::new(Mutex::new(None));
            let valid_arc: Arc<Mutex<Option<bool>>> = Arc::clone(&valid);
            let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", input);
            tokio::spawn(async move{
                let response = reqwest::get(&url).await;

                match response{
                    Ok(resp) => *valid_arc.lock().unwrap() = Some(resp.status() == 200),
                    Err(_) => *valid_arc.lock().unwrap() =  Some(false)}          
            });
            while *valid.lock().unwrap() == None{};
            let result = valid.lock().unwrap();
            match *result{
                Some(res) => ReturnType::IsValid(res),
                None => {eprint!{"Error returning validation response..."}; ReturnType::IsValid(false)},
            }   
    }
}

    impl MakeRequest for DbAPI{
        fn send_request(&self, input: &str) -> ReturnType{
            let url = format!("word-unscrambler-api-ade3e9ard4huhmbh.canadacentral-01.azurewebsites.net{}", input);
            let response_arc: Arc<Mutex<Vec<User>>> = Arc::clone(&self.friends);
            
            tokio::spawn(async move{
                let response = reqwest::get(&url).await;

                match response{
                    Ok(resp) => *response_arc.lock().unwrap() = {
                        let response_body: User = resp.json().await.unwrap();
                        self.friends = response_body;
                    },
                    Err(e) => eprint!("{}", e)}          
            });
            ReturnType::Error(None)
        }
    }
}