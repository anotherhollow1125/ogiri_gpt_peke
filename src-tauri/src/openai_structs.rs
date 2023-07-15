#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
    Other(String),
}

impl serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Role::System => serializer.serialize_str("system"),
            Role::User => serializer.serialize_str("user"),
            Role::Assistant => serializer.serialize_str("assistant"),
            Role::Other(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            _ => Ok(Role::Other(s)),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Choice {
    pub index: u64,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResponseBody {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_serialize_test() {
        let system_message = Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        };
        let user_message = Message {
            role: Role::User,
            content: "Hello!".to_string(),
        };
        let request_body = RequestBody {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![system_message, user_message],
        };

        let _json = serde_json::to_string(&request_body).unwrap();
    }

    #[test]
    fn request_deserialize_test() {
        let json = r#"{
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "system", "content": "You are a helpful assistant."}, {"role": "user", "content": "Hello!"}]
        }"#;
        let _request_body: RequestBody = serde_json::from_str(json).unwrap();
    }
}
