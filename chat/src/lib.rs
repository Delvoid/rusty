use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}
