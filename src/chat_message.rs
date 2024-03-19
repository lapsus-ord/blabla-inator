use gethostname::gethostname;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ChatMessage {
    timestamp: u128,
    username: String,
    message: String,
}

impl ChatMessage {
    pub fn new(message: String) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let username: String = gethostname().to_str().unwrap_or("").into();

        Self {
            timestamp: since_the_epoch.as_millis(),
            username,
            message,
        }
    }
}

impl ToString for ChatMessage {
    fn to_string(&self) -> String {
        format!("[{}][{}]{}", self.timestamp, self.username, self.message)
    }
}
