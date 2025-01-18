use std::process::exit;

#[derive(Clone, Debug)]
pub enum MessageType {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub r#type: MessageType,
    pub content: String,
}

impl Message {
    pub fn new(r#type: MessageType, content: &str) -> Self {
        Self {
            r#type,
            content: content.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageStack {
    stack: Vec<Message>,
}

impl MessageStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn insert_message(&mut self, message: Message) {
        self.stack.push(message);
    }

    pub fn list_all_messages(self) -> Vec<Message> {
        self.stack.clone()
    }

    pub fn list_info_messages(self) -> Vec<Message> {
        let mut result: Vec<Message> = Vec::new();

        for message in self.stack {
            match message.r#type {
                MessageType::Info => result.push(message.clone()),
                MessageType::Warning => {}
                MessageType::Error => {}
            }
        }

        result
    }

    pub fn list_warning_messages(self) -> Vec<Message> {
        let mut result: Vec<Message> = Vec::new();

        for message in self.stack {
            match message.r#type {
                MessageType::Info => {}
                MessageType::Warning => result.push(message.clone()),
                MessageType::Error => {}
            }
        }

        result
    }

    pub fn list_error_messages(self) -> Vec<Message> {
        let mut result: Vec<Message> = Vec::new();

        for message in self.stack {
            match message.r#type {
                MessageType::Info => {}
                MessageType::Warning => {}
                MessageType::Error => result.push(message.clone()),
            }
        }

        result
    }

    pub fn collect_error<T>(&mut self, input: Result<T, String>) -> T {
        match input {
            Ok(result) => return result,
            Err(code) => {
                let new_message = Message::new(MessageType::Error, code.as_str());

                self.insert_message(new_message);

                println!("RToy can't continue:");

                for error in self.clone().list_error_messages() {
                    println!("Error: {}", error.content);
                }

                exit(1)
            }
        }
    }
}
