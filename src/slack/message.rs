use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SlackMessage {
    pub channel: String,
    pub text: String,
}

impl SlackMessage {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"channel": "{}", "text": "{}"}}"#,
            self.channel, self.text
        )
    }
}

pub struct SlackErrorMessage;
