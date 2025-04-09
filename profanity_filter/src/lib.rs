pub struct Message {
    content: String, 
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Message { content, user }
    }

    pub fn send_ms(&self) -> Option<String> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(self.content.clone())
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), "user".to_string());
    
    match msg.send_ms() {
        Some(_) => Ok(message),
        None => Err("ERROR: illegal")
    }
}