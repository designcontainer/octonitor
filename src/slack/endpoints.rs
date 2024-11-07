use reqwest::Method;

// expects https://slack.com/api/ before
#[derive(Debug)]
pub struct SlackApiEndpoints {
    pub base: String,
    pub test: SlackApiEndpoint,
    pub message: SlackApiEndpoint,
} // SlackApiEndpoints

#[derive(Debug)]
pub struct SlackApiEndpoint {
    pub uri: String,
    pub method: Method,
} // SlackApiEndpoint

impl SlackApiEndpoints {
    pub fn new() -> Self {
        SlackApiEndpoints {
            base: String::from("https://slack.com/api/"),
            test: SlackApiEndpoint { // api.test
                uri: String::from("api.test"),
                method: Method::POST,
            },
            message: SlackApiEndpoint { // chat.postMessage
                uri: String::from("chat.postMessage"),
                method: Method::POST,
            }
        }
    }
} // SlackApiEndpoints
