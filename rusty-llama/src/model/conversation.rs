use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation{
    pub messages: Vec<Message>,
}

impl Conversation{
    pub fn new() -> Self{
        Conversation{
            messages: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    pub text: String,
    pub user: bool,
}