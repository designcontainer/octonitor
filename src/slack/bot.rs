use crate::slack::endpoints::SlackApiEndpoints;
use crate::slack::message::SlackMessage;

use std::fs;
use std::io;
use std::env;

use base64::{engine::general_purpose::STANDARD, Engine as _};
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
    pub fn new() -> Self {
        let token = SlackBot::load_token("SLACK");
        let auth_header = format!("Bearer {}", token);
        let endpoints = SlackApiEndpoints::new();
        let reqwest = Client::new();

        let bot = SlackBot {
            channel_id: String::from("C07RT7MTA8M"),
            auth_header: String::from(auth_header),
            reqwest_client: reqwest,
            endpoints: endpoints,
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
        let result = match response {
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
        let result = match response {
            Ok(gem) => gem,
            Err(e) => panic!("Could not send message. Error: {}", e),
        };
    } // send_message
    pub async fn get_installs(&self) {
        let client = Client::new();

        let slack_error_message = SlackMessage {
            channel: self.channel_id.clone(),
            text: String::new(),
        };

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

        let result = match response {
            Ok(val) => {
                dbg!(val);

            },
            Err(e) => {
                let emessage = format!("Could not fetch servers. Error {}", e);
                self.send_message(&emessage);
            },
        };
    } // get_installs
    fn check_installs() {

    } // check_installs()
} // SlackBot
