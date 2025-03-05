use crate::slack::endpoints::SlackApiEndpoints;
use crate::slack::message::SlackMessage;

use std::fs;
use std::io::{self, BufRead};
use std::path;
use std::env;

use base64::{
    engine::general_purpose::STANDARD,
    Engine as _
};
use serde_json::Value;
use reqwest::{
    Client,
    header::CONTENT_TYPE,
};

#[derive(Debug)]
pub struct SlackBot {
    pub channel_id: String,
    auth_header: String,
    pub reqwest_client: Client,
    pub endpoints: SlackApiEndpoints,
    pub ignored: Vec<String>,
} // SlackBot

impl SlackBot {
    /* LOADING AND OTHER BULLSHIT */
    fn load_token(var: &str) -> String {
        let vars = env::var(var);
        match vars {
            Ok(env) => env,
            Err(e) => panic!("Could not load environmental variables. Error: {}", e),
        }
    } // load_token

    // @todo this has to be renamed to ::init() or the panic shit needs to be removed
    pub fn new() -> Self {
        let token = SlackBot::load_token("SLACK");
        let auth_header = format!("Bearer {}", token);
        let endpoints = SlackApiEndpoints::new();
        let reqwest = Client::new();

        let to_ignore = Self::load_checklist();
        match to_ignore {
            Err(e) => panic!("I am so sorry, but ::new() can fail 🤫🧏.\n (if you're an ignoramous ::new() should never fail call it ::init() instead if that can happen)\n Error: {}", e),
            _ => {},
        }
        let bot = SlackBot {
            channel_id: String::from("C07RT7MTA8M"),
            auth_header: String::from(auth_header),
            reqwest_client: reqwest,
            endpoints: endpoints,
            ignored: to_ignore.unwrap(),
        };
        bot
    } // new

    /* MESSAGING AND THINGS THAT MATTER */
    pub async fn send_test(&self) {
        let response = self.reqwest_client
            .request(self.endpoints.test.method.clone(), format!("{}{}", self.endpoints.base, self.endpoints.test.uri))
            .header("Authorization", self.auth_header.clone())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await;
        /* let result = */ match response {
            Ok(val) => val,
            Err(e) => panic!("Response error: {}", e),
        };
    } // send_test
    pub async fn send_message(&self, message: &str) {
        let message = SlackMessage {
            channel: self.channel_id.clone(),
            text: String::from(message),
        };
        let json_string = message.to_json();
        let response = self.reqwest_client
            .request(self.endpoints.message.method.clone(), format!("{}{}", self.endpoints.base, self.endpoints.message.uri))
            .header("Authorization", self.auth_header.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(json_string)
            .send()
            .await;
        /* let result = */ match response {
            Ok(_) => {},
            Err(e) => println!("Error: Sending the message produced some problems: {}", e),
        };
    } // send_message
    pub async fn send_error(&self, error: &str) {
        let error = SlackMessage {
            channel: self.channel_id.clone(),
            text: String::from(format!("Error: {}", error)),
        };
        let json_string = error.to_json();
        let response = self.reqwest_client
            .request(self.endpoints.message.method.clone(), format!("{}{}", self.endpoints.base, self.endpoints.message.uri))
            .header("Authorization", self.auth_header.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(json_string)
            .send()
            .await;
        /* let result = */ match response {
            Ok(_) => {},
            Err(coal) => println!("Could not send message. Error: {}", coal),
        };
    } // send_error
    pub async fn get_installs(&self) {
        let client = Client::new();
        let url = "https://api.wpengineapi.com/v1/installs";

        // Here we need to load secrets
        // Important that : is part of the string that is encoded
        let auth = format!("{}:{}", SlackBot::load_token("WP_GUID"), SlackBot::load_token("WP_PASS"));
        let auth64 = STANDARD.encode(auth);
        let auth64_header = format!("Basic {}", auth64);

        let response = client
            .get(url)
            .header(CONTENT_TYPE, "text/plain")
            .header("Authorization", auth64_header)
            .send()
            .await;

        match response {
            Ok(res) => {
                match res.text().await {
                    Ok(body) => {
                        match serde_json::from_str::<Value>(&body) {
                            Ok(json) => {
                                dbg!(&json);
                                if let Some(value) = json.get("results") {
                                    SlackBot::check_installs(self, value).await;
                                } else {
                                    self.send_error("Cannot parse the JSON string").await;
                                }
                            }
                            Err(e) => self.send_error(&format!("Failed to parse JSON: {}", e)).await,
                        }
                    }
                    Err(e) => self.send_error(&format!("Failed to read response body, Error: {}", e)).await,
                }
            },
            Err(e) => {
                let emessage = format!("Could not fetch servers. Error {}", e);
                self.send_error(&emessage).await;
            },
        };
    } // get_installs
    async fn check_installs(&self, json_array: &Value) {
        dbg!(json_array);
        if let Some(array) = json_array.as_array() {
            'outer: for object in array {
                println!("we are looping");
                if let Some(status) = object.get("status") {
                    dbg!(status);
                    if status != "active" {
                        // Here there would be some check to see if it is on the ignore list
                        if let Some(name) = object.get("name"){
                            for site in &self.ignored {
                                if site == name {
                                    continue 'outer;
                                }
                            }
                            let message: String = format!("{} is down!", name);
                            self.send_message(&message).await;
                        }
                    }
                }
            }
        } else {
            panic!("&Value is not an array");
        }
    } // check_installs()
    fn load_checklist() -> io::Result<Vec<String>> {
        let mut tmp: Vec<String> = Vec::new();
        let path = path::Path::new("ignore.txt");
        let file = match fs::File::open(&path) {
            Err(e) => panic!("Could not open to.ignore: {}", e),
            Ok(file) => file,
        };
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Err(e) => return Err(e),
                Ok(line) => tmp.push(line),
            }
        }
        // read file line by line
        Ok(tmp)
    } // load_server_checklist()
} // SlackBot
