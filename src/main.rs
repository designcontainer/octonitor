mod slack;

use slack::bot::SlackBot;

use std::error::Error;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use reqwest::{
    Client,
    header::CONTENT_TYPE,
    Response,
    //StatusCode,
};

/// fetches the installations from wpengine using a token consisting of a uuid and pass string.
/// If it can't fetch from the server it sends a message in slack
async fn get_installs() -> Result<Response, Box<dyn Error>> {
    let client = Client::new();

    let url = "https://api.wpengineapi.com/v1/installs";

    // Needs to implement something which parses secrets for wp engine and implement it here.
    // Important that : is part of the string that is encoded
    let auth = format!("{}:{}", "", "");
    let auth64 = STANDARD.encode(auth);
    let auth64_header = format!("Basic {}", auth64);

    let response = client
        .get(url)
        .header(CONTENT_TYPE, "text/plain")
        .header("Authorization", auth64_header)
        .send()
        .await;

    let result = match response {
        Ok(val) => val,
        Err(e) => panic!("Response error: {}", e),
        // Send message in slack
    };

    Ok(result)
}


#[tokio::main]
async fn main() {
    let testbot = SlackBot::new();
    testbot.send_test().await;
    testbot.send_message("SENDIT").await;

    // match res_code {
    //     StatusCode::OK => println!(""),
    //     StatusCode::UNAUTHORIZED => println!(""),
    //     StatusCode::BAD_REQUEST => println!(""),
    //     StatusCode::FORBIDDEN => println!(""),
    //     StatusCode::INTERNAL_SERVER_ERROR => println!(""),
    //     other => println!("Status code not predicted: {}", other),
    // }
}
